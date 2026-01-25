# Change: Prioritize search results by usage history

## Why
- The current search is not sorted, so frequently run targets stay buried behind less relevant matches despite repeated use.
- Users expect their most common workflows to surface to the top of the list, which requires remembering what they actually launch.

## What Changes
- Introduce a persistent usage history that records how often each search result has been executed and store it inside `~/.stando`, creating the folder automatically when missing.
- Merge usage counts with the fuzzy-match score inside the search engine so results are ordered by frequency before relevance ties.
- Increment a result's count whenever the user launches it so the ranking adapts immediately across sessions.

## Impact
- Affected specs: search (new capability covering historical ranking)
- Affected code: `src/search.rs`, `src/app.rs`, plus a new history persistence component and related logging/tests.
