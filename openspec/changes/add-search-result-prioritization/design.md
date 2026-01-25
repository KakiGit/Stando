## Context
- Search results are currently ordered purely by fuzzy match score, so the UI repeats the same ranking even when the user consistently opens a particular application or file.
- The product already persists configuration under `~/.stando` via `xdg::BaseDirectories`, so placing search history alongside existing state keeps things centralized.
- We need a lightweight, resilient mechanism that records launches, survives restarts, and surfaces preferred targets without introducing heavy dependencies.

## Goals / Non-Goals
- **Goals:**
  - Persist run counts so ranking reflects actual usage across sessions.
  - Keep the data model simple (key/value map) and the file human-readable for debugging.
  - Ensure the folder is created automatically and write failures degrade gracefully.
- **Non-Goals:**
  - Track per-user analytics beyond local counts.
  - Implement an opaque recommendation engine or telemetry.

## Decisions
1. **Storage location and format**: Use a `search-history.json` file under `~/.stando` (via `xdg::BaseDirectories::with_prefix("stando")` or `HOME/.stando`) containing a JSON object mapping string identifiers to integer counters. JSON keeps debugging easy and avoids new dependencies.
2. **Identifier strategy**: Each `SearchResult` is reduced to a deterministic identifier (e.g., `app:/usr/share/applications/foo.desktop` or `file:/home/user/docs/report.pdf`) so the same result always hits the same bucket. Text results and AI responses skip history tracking.
3. **Ranking logic**: The search engine still computes fuzzy scores but sorts results by descending run count first, using fuzzy score as a secondary key when counts tie or are zero. Results with no history treat count as zero.
4. **Write strategy**: Increment counts before returning from `App::open_result`, save the file asynchronously or using blocking I/O in a dedicated helper so search path isn't blocked, and log but ignore write errors.

## Risks / Trade-offs
- **Corruption risk**: Concurrent writes could corrupt `search-history.json` if multiple Stando instances run simultaneously. Mitigate by writing atomically (write to temp file + rename) and assuming single-user scenario.
- **Performance**: Reading/writing JSON per launch adds overhead. Keep the file small and throttle writes by saving immediately but only on result launches.

## Migration Plan
1. On startup or before the first history write, ensure `~/.stando` exists and that `search-history.json` is created if missing (`{}` contents).
2. When running older versions, there is no history file; the new component treats missing file as empty history and creates it when a result is launched.

## Open Questions
- Should we expose the run count data in the UI (e.g., via tooltips)? Not required for this change.
