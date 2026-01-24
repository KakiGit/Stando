# Research: Add Debug Logging

## Decision 1: Logging instrumentation approach

- **Decision**: Use the existing `tracing` + `tracing-subscriber` stack to emit function entry/exit debug events.
- **Rationale**: `tracing` is already a dependency and provides structured spans/events, minimizing new dependencies and aligning with current logging initialization.
- **Alternatives considered**: `log` + `env_logger` with manual println-style logs; custom logger wrappers.

## Decision 2: Debug logging enablement

- **Decision**: Respect the existing runtime verbosity control (`--verbose` flag and `RUST_LOG`/EnvFilter) to enable or disable function-level debug logs.
- **Rationale**: This matches current CLI behavior and avoids introducing new configuration paths.
- **Alternatives considered**: New configuration file flag; compile-time feature gating.

## Decision 3: Privacy and sensitive data handling

- **Decision**: Emit only function name, entry/exit marker, outcome, and optional non-sensitive identifiers; avoid prompts, file contents, full paths, and secrets.
- **Rationale**: Aligns with constitution privacy constraints and feature requirement FR-005.
- **Alternatives considered**: Full payload logging or logging arbitrary function arguments.
