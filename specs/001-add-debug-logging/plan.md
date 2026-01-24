# Implementation Plan: Add Debug Logging

**Branch**: `001-add-debug-logging` | **Date**: 2026-01-24 | **Spec**: /home/kaki/Github/Stando/specs/001-add-debug-logging/spec.md  
**Input**: Feature specification from `/specs/001-add-debug-logging/spec.md`

## Summary

Add function-level debug logging across the Stando codebase using the existing logging stack, gated behind runtime configuration so logs are emitted only when debug is enabled and avoid sensitive data.

## Technical Context

**Language/Version**: Rust 2021 (Cargo package `stando` 0.1.0)  
**Primary Dependencies**: gtk4/libadwaita (UI), tracing + tracing-subscriber (logging), tokio (async), global-hotkey (hotkeys), reqwest (AI HTTP)  
**Storage**: Config file at `~/.config/stando/config.toml`; logs to standard output/error  
**Testing**: `cargo test` (no dedicated test harness yet)  
**Target Platform**: Linux desktop (GTK4)  
**Project Type**: Single binary application  
**Performance Goals**: Maintain responsive UI; negligible overhead when debug logging is disabled  
**Constraints**: Logs must not include secrets, prompts, file contents, or full paths unless debug logging is explicitly enabled; no added network calls  
**Scale/Scope**: Function-level logging across `src/*.rs` in the Stando repo

**Logging Conventions**:
- Log a debug entry on function entry and exit with function name and outcome.
- Mark outcome as error for fallible functions when returning an error.
- Never log prompts, file contents, full paths, API keys, or other secrets.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Keyboard-first UX: Logging adds no UI changes and remains non-blocking.
- Local-first/privacy: Debug logs are gated behind explicit enablement and must avoid sensitive data.
- Daemon reliability: Logging changes are additive and must not affect single-instance/hotkey behavior.
- Resource efficiency: Logging overhead is minimized; debug logging can be disabled.
- Linux integration: No change to GTK4/libadwaita or desktop artifacts.

**Post-Design Re-check**:
- No violations introduced; logging scope and privacy constraints remain aligned with constitution.

## Project Structure

### Documentation (this feature)

```text
specs/001-add-debug-logging/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── ai.rs
├── app.rs
├── config.rs
├── daemon.rs
├── hotkeys.rs
├── main.rs
├── search.rs
├── tray.rs
└── ui.rs
```

**Structure Decision**: Single Rust binary with modules in `src/`.

## Phase 0: Research

- Decide on logging instrumentation strategy using the existing `tracing` stack.
- Confirm debug logging toggles and environment configuration paths.
- Define privacy-safe logging rules for function-level instrumentation.

## Phase 1: Design & Contracts

- Model the debug log entry data shape and context identifiers.
- Document any API contracts (none expected; internal-only logging).
- Provide a quickstart for enabling debug logging during development.
- Update agent context with any new tech references (none expected).

## Phase 2: Planning

- Break down instrumentation by module and ensure coverage of entry/exit/error paths.
- Define verification steps for debug on/off behavior and privacy constraints.

## Complexity Tracking

No constitution violations requiring justification.
