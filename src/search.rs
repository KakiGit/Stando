use crate::history::UsageHistory;
use crate::logging;
use anyhow::{Context, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub enum SearchResult {
    File {
        path: PathBuf,
        name: String,
    },
    Application {
        name: String,
        exec: String,
        #[allow(dead_code)]
        icon: Option<String>,
        #[allow(dead_code)]
        desktop_path: PathBuf,
    },
    Text {
        content: String,
        name: String,
    },
}

impl SearchResult {
    pub fn display_name(&self) -> &str {
        let _log_guard = logging::function_guard("SearchResult::display_name");
        match self {
            SearchResult::File { name, .. } => name,
            SearchResult::Application { name, .. } => name,
            SearchResult::Text { name, .. } => name,
        }
    }

    pub fn history_id(&self) -> Option<String> {
        match self {
            SearchResult::File { path, .. } => Some(format!("file:{}", path.display())),
            SearchResult::Application { desktop_path, .. } => {
                Some(format!("app:{}", desktop_path.display()))
            }
            SearchResult::Text { .. } => None,
        }
    }

    #[allow(dead_code)]
    pub fn path(&self) -> Option<&Path> {
        let _log_guard = logging::function_guard("SearchResult::path");
        match self {
            SearchResult::File { path, .. } => Some(path),
            SearchResult::Application { desktop_path, .. } => Some(desktop_path),
            SearchResult::Text { .. } => None,
        }
    }
}

pub struct SearchEngine {
    matcher: SkimMatcherV2,
    file_index: Arc<RwLock<Vec<SearchResult>>>,
    app_index: Arc<RwLock<Vec<SearchResult>>>,
    history: Arc<UsageHistory>,
}

impl SearchEngine {
    pub fn new(history: Arc<UsageHistory>) -> Self {
        let _log_guard = logging::function_guard("SearchEngine::new");
        Self {
            matcher: SkimMatcherV2::default(),
            file_index: Arc::new(RwLock::new(Vec::new())),
            app_index: Arc::new(RwLock::new(Vec::new())),
            history,
        }
    }

    pub async fn index_files(&self, search_paths: &[String]) -> Result<()> {
        let log_guard = logging::function_guard("SearchEngine::index_files");
        let result = async {
            let mut results = Vec::new();

            for path_str in search_paths {
                let path = Path::new(path_str);
                if !path.exists() {
                    continue;
                }

                for entry in WalkDir::new(path)
                    .follow_links(false)
                    .max_depth(10)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        let path = entry.path().to_path_buf();
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            let name_str = name.to_string();
                            results.push(SearchResult::File {
                                path,
                                name: name_str,
                            });
                        }
                    }
                }
            }

            *self.file_index.write().await = results;
            Ok(())
        }
        .await;
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub async fn index_applications(&self) -> Result<()> {
        let log_guard = logging::function_guard("SearchEngine::index_applications");
        let result = async {
            let mut results = Vec::new();

            let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
            let app_dirs = vec![
                "/usr/share/applications".to_string(),
                "/usr/local/share/applications".to_string(),
                format!("{}/.local/share/applications", home_dir),
            ];

            for app_dir in app_dirs {
                let dir = Path::new(&app_dir);
                if !dir.exists() {
                    continue;
                }

                for entry in WalkDir::new(dir)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        let path = entry.path();
                        if path.extension().and_then(|e| e.to_str()) == Some("desktop") {
                            if let Ok(app) = self.parse_desktop_file(path) {
                                results.push(app);
                            }
                        }
                    }
                }
            }

            *self.app_index.write().await = results;
            Ok(())
        }
        .await;
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    fn parse_desktop_file(&self, path: &Path) -> Result<SearchResult> {
        let log_guard = logging::function_guard("SearchEngine::parse_desktop_file");
        let result = (|| {
            let content = std::fs::read_to_string(path).context("Failed to read desktop file")?;

            let mut name = None;
            let mut exec = None;
            let mut icon = None;

            for line in content.lines() {
                if let Some(value) = line.strip_prefix("Name=") {
                    name = Some(value.trim().to_string());
                } else if let Some(value) = line.strip_prefix("Exec=") {
                    exec = Some(value.trim().to_string());
                } else if let Some(value) = line.strip_prefix("Icon=") {
                    icon = Some(value.trim().to_string());
                }

                if name.is_some() && exec.is_some() {
                    break;
                }
            }

            let name = name.unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string()
            });

            let exec = exec.unwrap_or_default();

            Ok(SearchResult::Application {
                name,
                exec,
                icon,
                desktop_path: path.to_path_buf(),
            })
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub async fn search(&self, query: &str, max_results: usize) -> Vec<SearchResult> {
        let _log_guard = logging::function_guard("SearchEngine::search");
        // if query.is_empty() {
        //     return Vec::new();
        // }

        let mut results = Vec::new();

        // Search files
        // {
        //     let file_index = self.file_index.read().await;
        //     for result in file_index.iter() {
        //         if let Some(score) = self.matcher.fuzzy_match(result.display_name(), query) {
        //             results.push((score, result.clone()));
        //         }
        //     }
        // }

        // Search applications
        {
            let app_index = self.app_index.read().await;
            for result in app_index.iter() {
                if let Some(score) = self.matcher.fuzzy_match(result.display_name(), query) {
                    results.push((score, result.clone()));
                }
            }
        }

        let counts = self.history.snapshot().await;
        results.sort_by(|a, b| {
            let a_result = &a.1;
            let b_result = &b.1;
            let a_count = a_result
                .history_id()
                .and_then(|id| counts.get(&id).copied())
                .unwrap_or(0);
            let b_count = b_result
                .history_id()
                .and_then(|id| counts.get(&id).copied())
                .unwrap_or(0);
            match b_count.cmp(&a_count) {
                Ordering::Equal => b.0.cmp(&a.0),
                ord => ord,
            }
        });
        results
            .into_iter()
            .take(max_results)
            .map(|(_, result)| result)
            .collect()
    }

    pub async fn find_by_reference(&self, reference: &str) -> Option<SearchResult> {
        let _log_guard = logging::function_guard("SearchEngine::find_by_reference");
        // Remove @ symbol if present
        let query = reference.strip_prefix('@').unwrap_or(reference);

        // Search in both indexes
        // let file_index = self.file_index.read().await;
        // for result in file_index.iter() {
        //     if result
        //         .display_name()
        //         .to_lowercase()
        //         .contains(&query.to_lowercase())
        //     {
        //         return Some(result.clone());
        //     }
        // }
        // drop(file_index);

        let app_index = self.app_index.read().await;
        for result in app_index.iter() {
            if result
                .display_name()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                return Some(result.clone());
            }
        }

        None
    }
}
