this is a proposed example for a file structure

src/
├── main.rs
├── app.rs                # App setup & plugin registration
│
├── core/                 # Engine-agnostic game foundations
│   ├── mod.rs
│   ├── states.rs         # AppState, GameState
│   ├── time_scale.rs     # Deadeye / slowmo
│   ├── camera.rs
│   └── assets.rs
│
├── gameplay/             # Runtime game systems
│   ├── mod.rs
│   ├── player/
│   ├── weapons/
│   ├── enemies/
│   ├── combat/
│   ├── movement/
│   └── world/
│
├── narrative/            # Story, dialogue, timeline
│   ├── mod.rs
│   ├── dialogue.rs
│   ├── timeline.rs
│   ├── triggers.rs
│   └── scenes.rs
│
├── ui/                   # Runtime UI only
│   ├── mod.rs
│   ├── main_menu.rs
│   ├── hud.rs
│   ├── pause.rs
│   └── dialogue_ui.rs
│
├── editor/               # Dev-only tools
│   ├── mod.rs
│   ├── editor_state.rs
│   ├── gizmos.rs
│   ├── selection.rs
│   ├── level_editor.rs
│   └── save_load.rs
│
└── dev/                  # Debug & cheats (optional)
    ├── mod.rs
    └── debug_ui.rs
