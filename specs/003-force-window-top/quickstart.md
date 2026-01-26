# Quickstart: Force Window Top

1. **Start Stando**: Launch the application in your target WM (e.g., i3, sway, or hyprland) and confirm the UI renders its main window.  
2. **Activate floating mode**: Click the new floating-mode indicator (or use the keyboard shortcut once defined) so the window requests to stay on top. Look for the visual cue that confirms the mode is active.  
3. **Exercise workspace/display changes**:  
   - Switch to other workspaces and bring random windows into focus.  
   - Drag the Stando window across monitors if multiple displays exist.  
   - If possible, open a fullscreen client to observe reassertion behavior.  
4. **Verify persistence**: Close Stando while floating mode is on, restart the app, and ensure it resumes with floating mode enabled without extra interaction.  
5. **Test fallback messaging**: Enable floating mode on a workspace or compositor that is known to reject stacking overrides, and verify the UI explains the restriction while keeping the window usable.
