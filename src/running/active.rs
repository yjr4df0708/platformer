use bevy::prelude::*;
use crate::{
    cleanup_system,
    mechanics::{
        *,
        effect::EffectSystemSet,
    },
};
use super::{
    RunningState,
    RunningEntity,
};

#[derive(Component)]
#[require(RunningEntity)]
pub struct ActiveEntity;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RunningState::Active), on_enter)
        .add_systems(FixedUpdate, (
            ManualControl::system,
            caster_system,
        ).chain().run_if(in_state(RunningState::Active)))
        .configure_sets(FixedUpdate, EffectSystemSet
            .after(caster_system)
            .run_if(in_state(RunningState::Active))
        )
        .add_systems(OnExit(RunningState::Active), (cleanup_system::<ActiveEntity>).chain())
    ;
}

fn on_enter() {}