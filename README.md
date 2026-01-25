# Bevy Template

Game or interactive application built with the Bevy engine.

## Stack

- **Engine**: Bevy 0.15+
- **ECS**: Entity Component System (Bevy's core paradigm)
- **Assets**: bevy_asset_loader for state-based loading

## Features

- Game state management (Loading, Menu, InGame, Paused)
- Asset loading with progress tracking
- Input handling abstraction
- 2D camera setup (swap for 3D as needed)
- Debug overlay (dev builds)
- Strict Clippy lints
- Optimized release builds
- Fast compiles in dev (dynamic linking)

## Structure

```
src/
  main.rs           # Entry, app builder
  states.rs         # Game states
  plugins/
    mod.rs
    loading.rs      # Asset loading plugin
    menu.rs         # Menu plugin
    game.rs         # Core game plugin
  components/       # ECS components
  systems/          # ECS systems
  resources/        # ECS resources
assets/
  sprites/
  audio/
  fonts/
Cargo.toml
```

## Dependencies

- bevy (default features for quick start)
- bevy_asset_loader
- rand (if needed)

### Dev Dependencies

- bevy (dynamic_linking feature for fast compiles)

## Architecture

```
States:  Loading → Menu → InGame ↔ Paused
                    ↑        │
                    └────────┘ (back to menu)
```

## Commands

```sh
cargo run                           # Dev (slow first compile)
cargo run --features bevy/dynamic_linking  # Dev (fast recompiles)
cargo build --release               # Release build
cargo test                          # Tests
```

## Performance Tips

- Use `--features bevy/dynamic_linking` during dev
- Enable LTO only for release
- Consider `bevy_embedded_assets` for distribution

## TODO

- [ ] Cargo.toml (with dynamic_linking feature)
- [ ] src/main.rs (app builder, plugins)
- [ ] src/states.rs
- [ ] src/plugins/loading.rs
- [ ] src/plugins/menu.rs
- [ ] src/plugins/game.rs
- [ ] Basic asset structure
- [ ] .cargo/config.toml (fast compiles)
