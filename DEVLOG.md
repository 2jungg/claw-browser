# DEVLOG

## 2026-02-04
- Initialized ClawBrowser project.
- Added dependencies: `reqwest`, `tokio`.
- Created basic `main.rs` to fetch Google and verify connectivity.
- Verified project with `cargo check`.

## 2026-02-04 (Phase 3)
- Implemented CSS parsing structures and basic Layout engine.
- Added dependencies: `cssparser`, `selectors`.
- Created `src/css.rs` to define CSS rule structures (StyleSheet, Rule, Selector, Declaration, Value).
- Created `src/style.rs` to implement the Style Tree, pairing DOM nodes with computed styles based on a default stylesheet.
- Created `src/layout.rs` to implement a basic Box Model layout engine (block layout, width/height calculation, positions).
- Updated `src/main.rs` to show the Layout Tree with (x, y, width, height) information.
- Successfully printed the Layout Tree for `https://example.com`.
