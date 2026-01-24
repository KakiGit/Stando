<!-- Sync Impact Report:
- Version change: N/A (template) -> 0.1.0
- Modified principles: N/A (template placeholders instantiated)
- Added sections: Core Principles, Security & Configuration Constraints, Development Workflow, Governance
- Removed sections: None
- Templates requiring updates:
  - .specify/templates/plan-template.md ✅ updated
  - .specify/templates/spec-template.md ✅ up-to-date (no change)
  - .specify/templates/tasks-template.md ✅ updated
- Follow-up TODOs:
  - TODO(RATIFICATION_DATE): original adoption date not recorded
-->
# Stando Constitution

## Core Principles

### I. Keyboard-First, Low-Latency UX
All core flows MUST be fully operable from the keyboard (open, select, toggle AI,
close). The UI thread MUST remain responsive; long-running work MUST run async,
support cancellation, and stream results progressively. Rationale: Stando is a
search tool; any lag breaks the primary value.

### II. Local-First & Privacy by Default
File and application discovery MUST be local by default. Network calls are ONLY
allowed when AI mode is explicitly enabled and a user-provided API key is
configured. Logs MUST avoid prompts, file contents, or full paths unless the
user explicitly enables debug logging. Rationale: user trust and privacy are
non-negotiable.

### III. Resilient Daemon & Hotkey Reliability
Daemon mode MUST be single-instance, recoverable, and able to start/stop cleanly.
The global hotkey MUST open the UI even when hidden; failure states MUST be
observable via logs or notifications. Rationale: an always-on launcher must be
dependable.

### IV. Resource Efficiency
The idle daemon MUST avoid busy polling and remain lightweight in CPU and memory.
Indexing/search MUST be incremental, bounded, and avoid full-disk scans on
startup. Rationale: the app runs continuously and must stay unobtrusive.

### V. Linux Desktop Integration
UI work MUST stay within GTK4/libadwaita conventions, and desktop integration
artifacts (desktop file, tray behavior, hotkeys) MUST remain accurate. Linux is
the target platform; Arch Linux is the reference, but implementation SHOULD
avoid distro-specific assumptions. Rationale: consistent, native integration is
core to usability.

## Security & Configuration Constraints

- Configuration lives at `~/.config/stando/config.toml`; secrets MUST never be
  committed to the repo.
- OpenAI keys MUST be loaded from config (or environment) and missing keys MUST
  disable AI features without failing core search.
- Search paths MUST be user-configurable and default to the user's home when
  unset.
- Telemetry or analytics are disallowed unless explicitly specified in a feature
  spec and surfaced to the user.

## Development Workflow

- Run `cargo fmt` and `cargo clippy` for Rust changes; new warnings MUST be
- Run `cargo test` and ensure all tests pass before merging; new or failing
  resolved.
- Run `cargo check` to verify type correctness; new or unresolved errors MUST be
  resolved; new warnings MUST be resolved.
- Changes to search ranking, hotkey handling, or AI prompts MUST include tests
  or a documented manual verification plan in the spec/plan.
- User-facing changes MUST update `README.md` and any configuration examples.
- Performance-sensitive work MUST note expected impact and basic measurement
  approach in the plan.

## Governance

- This constitution supersedes other templates or ad-hoc practices.
- Amendments require an updated Sync Impact Report, a rationale, and reviewer
  approval.
- Versioning follows semantic versioning: MAJOR for breaking governance changes,
  MINOR for new principles or material expansions, PATCH for clarifications.
- All specs/plans MUST include a Constitution Check, and PRs MUST confirm
  compliance before merge.
- Runtime guidance lives in `README.md` and must be kept consistent with this
  document.

**Version**: 0.1.0 | **Ratified**: TODO(RATIFICATION_DATE): original adoption date not recorded | **Last Amended**: 2026-01-24
