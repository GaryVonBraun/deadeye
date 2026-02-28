ui/
├── mod.rs
├── plugin.rs
├── common/
│   ├── mod.rs
│   ├── styles.rs          // Shared colors, fonts, spacing, theme
│   ├── components.rs      // Base UI components (UiButton, UiText, etc.)
│   ├── bundles.rs         // Reusable UI bundles (BaseButtonBundle)
│   └── systems/
│       ├── mod.rs
│       └── visual_feedback.rs   // Generic hover/press visuals
│
├── main_menu/
│   ├── mod.rs
│   ├── components.rs      // Screen-specific markers/components
│   ├── styles.rs          // Optional screen overrides
│   └── systems/
│       ├── mod.rs
│       ├── layout.rs      // Spawns and structures the UI tree
│       └── behavior.rs    // Handles button actions + screen logic
│
├── settings/
│   ├── mod.rs
│   ├── components.rs
│   └── systems/
│       ├── mod.rs
│       ├── layout.rs
│       └── behavior.rs