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
- Strict Clippy lints
- Optimized release builds
- Fast compiles in dev (dynamic linking + LLD)

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

## Architecture

```
States:  Loading → Menu → InGame ↔ Paused
                    ↑        │
                    └────────┘ (back to menu)
```

## Commands

```sh
cargo run                    # Dev build
cargo run --features dev     # Dev with dynamic linking (fast recompiles)
cargo build --release        # Release build
cargo test                   # Tests
```

## Performance Tips

- Use `--features dev` during development for faster iteration
- The `.cargo/config.toml` enables LLD linker for faster linking
- Release builds use LTO for maximum optimization
- Consider `bevy_embedded_assets` for distribution
