# ai-mode Specification

## Purpose
Allow keyboard-focused users to start the search window already in AI mode so they can immediately leverage the `@`-prefixed workflow without an extra toggle.

## ADDED Requirements
### Requirement: CLI flag launches Stando with AI mode enabled
The `stando` CLI SHALL accept a boolean switch (`--ai-mode` with alias `-a`) that leaves the AI mode indicator set to active, seeds the internal `ai_mode` state, and otherwise behaves exactly as if the user had pressed the in-window AI toggle before the window became visible.

#### Scenario: Launching with `stando --ai-mode` activates AI mode immediately
- **WHEN** the user runs `stando --ai-mode`
- **THEN** the search window appears with the AI indicator lit and the search entry focused before any keystrokes
- **AND** typing a query that begins with `@` is handled by the AI query path (displaying the AI loading result and filtering only the `@` suffix)
- **AND** typing a query without an `@` prefix still follows the AI-mode reset-content path (no normal fuzzy ranking is shown until the user explicitly leaves AI mode)

#### Scenario: Launching without the flag preserves the default search state
- **WHEN** the user runs `stando` without `--ai-mode`
- **THEN** the AI indicator remains off and the search entry behaves exactly as it does today, with Control+i still available to enable AI mode on demand
