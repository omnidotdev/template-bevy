//! Asset loading plugin.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::states::GameState;

/// Plugin for loading game assets.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAssets>(),
        );
    }
}

/// Collection of game assets loaded during the loading state.
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Example: load a texture.
    // #[asset(path = "sprites/player.png")]
    // pub player: Handle<Image>,
}
