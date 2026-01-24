# Quickstart: Add Debug Logging

## Goal

Verify that function-level debug logs appear when enabled and are absent when disabled.

## Steps

1. Run the app with debug logging enabled (example: `stando --verbose` or `RUST_LOG=debug stando`).
2. Trigger a user action (open the UI, type a query, launch a result).
3. Confirm logs show function entry/exit events and error outcomes, without sensitive data.
4. Run the app without debug logging enabled and repeat the action.
5. Confirm function-level debug entries are not emitted.
