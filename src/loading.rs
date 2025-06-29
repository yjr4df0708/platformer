use bevy::prelude::*;
use super::{cleanup_system, GameState};

#[derive(Component)]
struct LoadingEntity;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Loading), on_enter)
        .add_systems(OnExit(GameState::Loading), cleanup_system::<LoadingEntity>)
    ;
}

fn on_enter(
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Running);
}