//! Core game plugin.

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for core gameplay.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(
                Update,
                (handle_input, handle_pause).run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, handle_unpause.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::InGame), cleanup_game);
    }
}

#[derive(Component)]
struct GameRoot;

fn setup_game(mut commands: Commands) {
    commands.spawn((
        GameRoot,
        Text::new("Game running - Press ESC to pause"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
    ));

    // TODO: Spawn game entities here.
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Return to menu on Backspace.
    if keyboard.just_pressed(KeyCode::Backspace) {
        next_state.set(GameState::Menu);
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_unpause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
