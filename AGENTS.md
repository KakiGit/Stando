# Stando Development Guidelines

## General Principles

**MUST** run the listed [Commands](#commands) before finalizing the changes.
**MUST** follow the [Code Style](#code-Style) conventions.
**MUST** update [Documentations](#documentations) if needed.

## Code Style

Rust 2021: Follow standard conventions
**MUST** fix any warnings or errors they report.
**MUST NOT** have dead code or unused imports.
**MUST** follow idiomatic Rust practices.
Each function **MUST** have appropriate logging statements for tracing execution flow.
Each function **MUST** handle errors gracefully and log error conditions.
Each function **MUST** serve a single, well-defined purpose.

## Documentations

* [README.md](./README.md) project overview, build/install instructions.
* [SPEC.md](./SPEC.md) feature specifications and design decisions.

## Commands

```bash
cargo test
cargo check
cargo clippy
cargo fmt
cargo build
```

<!-- MANUAL ADDITIONS START -->
## TOOLS AND KNOWLEDGE
Your have outdated knowledge please look for the latest info in the internet when in doubt with call to `searxng.search_searxng` MCP tool.
<!-- MANUAL ADDITIONS END -->
