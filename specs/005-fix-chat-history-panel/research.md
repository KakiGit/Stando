# Research Notes — AI Mode Chat Grouping

## Stored history must include chat hashes for grouping
- Decision: keep `UsageHistory` backed by per-round JSON records but extend `ChatHistoryRecord` with an optional `chat_hash` string and treat each persisted row as a `ConversationRound`. During load we group rounds into a new `ChatGroup` entity keyed by `hash` (or `Recent Chat` when missing) before handing the data to `HistoryPanelState`.
- Rationale: The spec insists on chat-level grouping and a fallback for missing hashes while we already persist each round to `chat-history.json`. Adding a nullable hash field and grouping on load avoids a large schema rewrite or a migration path for existing files while still allowing metadata (last speaker/timestamp) to be derived.
- Alternatives considered: (a) Serialize pre-aggregated chat entries on disk — rejected because it would double the data model, demand migration, and duplicate `UsageHistory` logic; (b) Keep the current per-round disk format without hash metadata and infer grouping only from in-memory heuristics — rejected because it cannot reliably satisfy FR-001/FR-006 without a deterministic hash field.

## List/detail UI surfaces entire chat timeline with metadata
- Decision: build list rows (`ChatSummary`) from each `ChatGroup`, derive metadata (hash, last speaker, latest timestamp), and render the detail pane as a concatenated chronological timeline of `ConversationRound`s (each prefixed with role and timestamp) inside the existing `TextView`.
- Rationale: FR-002/FR-003/FR-004 require metadata visibility and a full, chronologically ordered timeline. Reusing the `TextView`’s simple text output keeps the rendering path lightweight while still letting us insert dividers or timestamps, and the existing `ListBox` already supplies keyboard focus management.
- Alternatives considered: (a) Building nested GTK containers per round (e.g., buttons or labels) — rejected because it adds rendering overhead and complicates keyboard nav; (b) Showing only the last round per chat — rejected because it violates the requirement to show every round and would confuse users resuming multi-round conversations.

## Sync preserves selection and keeps orphaned rounds grouped
- Decision: when history updates arrive (initial load or background sync), regroup by hash, retain the previously selected chat hash if still available, and treat rounds without a hash as part of a synthetic `Recent Chat` entry; selection updates happen by matching chat hash rather than IDs tied to individual rounds.
- Rationale: FR-005 and SC-004 mandate refreshing chat-level entries without losing the selected chat and without orphaning rounds. Matching by hash ensures continuity even as new rounds append, and the synthetic fallback prevents data loss when hashes are absent.
- Alternatives considered: (a) Always select the newest round after a refresh — violates the requirement to preserve selection; (b) Drop rounds without hashes completely — violates the “no orphaned rounds” constraint.
