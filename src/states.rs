//! Game state definitions.

use bevy::prelude::*;

/// Top-level game states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// Loading assets.
    #[default]
    Loading,
    /// Main menu.
    Menu,
    /// Active gameplay.
    InGame,
    /// Game paused.
    Paused,
}
