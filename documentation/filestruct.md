src/
│
├── main.rs
│
├── core/                # engine + global systems
│   ├── mod.rs
│   ├── states.rs
│   ├── time.rs
│   └── logging.rs
│
├── simulation/          # gameplay rules
│   ├── mod.rs
│   ├── systems/
│   │   ├── ai.rs
│   │   ├── needs.rs
│   │   ├── movement.rs
│   │   └── interactions.rs
│   └── resources/
│       ├── simulation_time.rs
│       └── economy.rs
│
├── world/               # environment and map
│   ├── mod.rs
│   ├── map/
│   ├── terrain/
│   ├── weather/
│   └── objects/
│
├── actor/
│   ├── mod.rs
│   │
│   ├── bundles/
│   │   ├── actor_bundle.rs
│   │   ├── npc_bundle.rs
│   │   ├── player_bundle.rs
│   │   └── animal_bundle.rs
│   │
│   ├── components/
│   │   ├── identity.rs
│   │   ├── stats.rs
│   │   ├── needs.rs
│   │   ├── locomotion.rs
│   │   └── inventory.rs
│   │
│   ├── ai/
│   │   ├── behaviour_tree.rs
│   │   ├── goals.rs
│   │   └── sensors.rs
│   │
│   ├── control/
│   │   ├── player_controller.rs
│   │   └── npc_controller.rs
│   │
│   ├── visuals/
│   │   ├── humanoid.rs
│   │   ├── animal.rs
│   │   └── layering.rs
│   │
│   └── factory/
│       ├── actor_factory.rs
│       ├── npc_factory.rs
│       ├── player_factory.rs
│       └── animal_factory.rs
│
├── ui/
└── assets/
