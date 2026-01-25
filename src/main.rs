//! Game entry point and app builder.

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;
use plugins::{GamePlugin, LoadingPlugin, MenuPlugin};
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project-name}}".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((LoadingPlugin, MenuPlugin, GamePlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
