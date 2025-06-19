use bevy::prelude::*;
use super::{cleanup_system, GameState};

#[derive(Component)]
struct RunningEntity;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum RunningState{
    #[default]
    Disabled,
    Running,
    Editing,
    Paused,
}

mod running;
mod editing;
mod paused;

pub fn plugin(app: &mut App) {
    app
        .init_state::<RunningState>()
        .add_systems(OnEnter(GameState::Running), on_enter)
        .add_systems(OnExit(GameState::Running), cleanup_system::<RunningEntity>)
    ;
}

#[derive(Component)]
struct Health(u64, u64);//current / max

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Enemy;

fn on_enter(
    mut running_state: ResMut<NextState<RunningState>>,
) {
    running_state.set(RunningState::Running);
}