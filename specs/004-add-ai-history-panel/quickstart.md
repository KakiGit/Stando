# Quickstart: AI History Panel

1. Launch Stando with AI mode enabled (`stando --ai-mode` or toggle AI mode inside the window) so the AI panel replaces the bottom results area.
2. With existing AI conversation history, confirm the right-hand list displays each entry with a timestamp/role label and the left-hand pane shows the selected entry’s full content immediately after the panel appears. Use Up/Down to change selection and verify the detail text updates as the same keystrokes are processed within a second.
3. Start a fresh conversation (no prior messages). Confirm the list shows the empty-state placeholder explaining how history entries appear and the left-side text explains that a selection will show when entries exist.
4. Submit a new prompt (`@` query or AI command) and wait for the AI response. Verify the new entry appends to the bottom of the list, automatically becomes selected, and its detail text renders without requiring manual refresh; also ensure the list scrolls if the entry is out of view.
5. While new AI responses arrive, keep the previously selected entry in focus and verify the list/text stay synchronized (most recent entry remains selected when appropriate, empty-state message disappears).
