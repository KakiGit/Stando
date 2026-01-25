<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# Stando Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-24

## Active Technologies
- Rust 2021 + gtk4, libadwaita, glib, gio (002-fix-ai-shortcut)

- Rust 2021 (Cargo package `stando` 0.1.0) + gtk4/libadwaita (UI), tracing + tracing-subscriber (logging), tokio (async), global-hotkey (hotkeys), reqwest (AI HTTP) (001-add-debug-logging)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2021 (Cargo package `stando` 0.1.0): Follow standard conventions

## Recent Changes
- 002-fix-ai-shortcut: Added Rust 2021 + gtk4, libadwaita, glib, gio

- 001-add-debug-logging: Added Rust 2021 (Cargo package `stando` 0.1.0) + gtk4/libadwaita (UI), tracing + tracing-subscriber (logging), tokio (async), global-hotkey (hotkeys), reqwest (AI HTTP)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
