# Research: AI History Panel

## Decisions

- **Decision**: Use a horizontal `gtk4::Paned` container with a read-only `TextView` on the left and a `ScrolledWindow` containing the existing `ListBox` on the right, with arrow keys handled via the list's `EventControllerKey` to keep focus in the panel.
  - **Rationale**: `GtkPaned` preserves the desired split layout, handles resize affordances, and keeps the existing `ListBox` widgets that already render search/AI responses; the `TextView` on the left can be kept read-only for reference content, matching FR-001/FR-004. Arrow key handling stays keyboard-first without moving focus to other widgets.
  - **Alternatives considered**: Building the layout with a single `gtk4::Box` and manual sizing logic would require more work to mimic pane resizing; using `gtk4::Grid` or `GtkListView` would change the current styling and selection behavior that already works for the search list.

- **Decision**: Introduce a shared `Arc<RwLock<Vec<ChatHistoryEntry>>>` inside `App`/`SearchWindow` to represent chat history entries (`id`, `timestamp`, `role`, `summary`, `content`, `selected`), and broadcast updates whenever AI requests finish.
  - **Rationale**: The existing `SearchResult` list reflects transient query results, so a dedicated structure stores chronological conversation entries and lets the UI render both the list (metadata) and the detail text (full content). A shared `RwLock` keeps the asynchronous AI service and the GTK UI synchronized without blocking. This supports FR-001 through FR-005 and the success criteria around new entries appearing without manual refresh.
  - **Alternatives considered**: Reusing `SearchResult` history or `UsageHistory` alone was not sufficient because they do not expose metadata like role/timestamps or guarantee the chronological ordering and selection persistence required by the spec.

- **Decision**: Keep UI updates on the GTK main context by using `glib::MainContext::default().spawn_local` whenever chat history or selection changes, while keeping AI response processing in background threads/tasks (via `tokio`/`thread::spawn`).
  - **Rationale**: GTK requires main-thread updates, so shipping async results through `spawn_local` maintains the responsive UI mandated by the constitution and performance goals (2 s visibility, 1 s selection). Tokio background workers already fetch AI responses, so bridging the two with lightweight main-context closures avoids blocking the UI while ensuring list and detail views stay synchronized.
  - **Alternatives considered**: Polling on the main thread or performing synchronous AI calls would hurt responsiveness; trying to update widgets from Tokio tasks risks GTK warnings and race conditions.
