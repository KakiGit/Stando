## 1. Implementation
- [x] 1.1 Add a history persistence component that reads and writes a map of result identifiers to run counts inside `~/.stando` and guarantees the directory exists before writing.
- [x] 1.2 Expose the stored counts to `SearchEngine::search` so it can sort results by history before breaking ties with fuzzy scores.
- [x] 1.3 Instrument `App::open_result` (and any other execution paths) to increment the appropriate run count immediately when a result launches and persist the update.
- [x] 1.4 Add logging or telemetry so failures to read/write the history file are visible and do not crash the app.

## 2. Validation
- [x] 2.1 `cargo test`
- [x] 2.2 Manual smoke test: run the app, execute several search targets, and confirm the history file is created under `~/.stando`, counts increase, and later searches surface the most-run items first.
