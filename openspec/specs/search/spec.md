# search Specification

## Purpose
TBD - created by archiving change add-search-result-prioritization. Update Purpose after archive.
## Requirements
### Requirement: Sort search results by historic usage
The search results displayed for a query SHALL be ordered first by how many times each result has been launched in the past (descending), and then by the existing fuzzy-match score so the order still reflects relevance when counts are equal.

#### Scenario: Frequently run result ranks above new matches
- **WHEN** the user has launched `App A` five times and `App B` zero times in prior sessions
- **AND** the user types a query that matches both apps with identical fuzzy-match scores
- **THEN** `App A` appears above `App B` because its historic run count is higher
- **AND** subsequent selections still obey the fuzzy order when counts tie

### Requirement: Persist usage counts under `~/.stando`
The system SHALL persist a map of search-result identifiers to launch counts inside a file located in the user's `~/.stando` directory, creating the directory and file automatically if either is missing, and load the map when the app starts so counts survive restarts.

#### Scenario: History folder created on demand
- **WHEN** Stando launches for the first time and `~/.stando` does not exist
- **THEN** Stando creates the directory and an empty history file before attempting to write any counts
- **AND** later searches read from the new file and treat missing entries as zero

### Requirement: Increment history immediately when a result is executed
When a search result is opened (file or application), the system SHALL increment its run count in the persistent history and persist the updated count before the next search so the ranking reflects the action immediately.

#### Scenario: Launch increases count for next search
- **WHEN** the user executes `App C` from the search window
- **AND** the updated history file is successfully saved
- **THEN** the next search that matches `App C` shows it ranked higher because its count has incremented

