use bevy::prelude::*;
use crate::{
    //extrapolate::ExtrapolateTransform,
    interpolate::{
        interpolate_transform_system,
        InterpolateTransform,
        OldTransform
    },
    mechanics::{
        action::Action,
        damage::Damage,
        projectile::{
            Lifetime,
            ProjectileGrace
        },
        *,
    },
    state::{
        CursorAngleRes,
        GameMode,
    },
    MainCamera,
};
use super::{cleanup_system, GameState};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
struct RunningEntity;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum RunningState{
    #[default]
    Disabled,
    Active,
    Editing,
    Paused,
}

mod active;
mod editing;
mod paused;

pub fn plugin(app: &mut App) {
    app
        .init_state::<RunningState>()
        .add_systems(OnEnter(GameState::Running), on_enter)
        .add_systems(FixedPreUpdate, (
            CursorAngleRes::system,
            ProjectileGrace::system,
            Lifetime::system,
        ).run_if(in_state(RunningState::Active)))
        .add_systems(Last, lerp_camera_to_player.after(interpolate_transform_system).run_if(in_state(GameState::Running)))
        .add_systems(OnExit(GameState::Running), cleanup_system::<RunningEntity>)
        .add_plugins(active::plugin)
    ;
}

fn on_enter(
    mut commands: Commands,
    mut running_state: ResMut<NextState<RunningState>>,
    gamemode: Res<GameMode>,
) {
    //match the gamemode, campaign, endless or testing
    match gamemode.into_inner() {
        GameMode::Testing => {
            running_state.set(RunningState::Active);
            let temp = [
                commands.spawn((
                    InterpreterState {
                        actions: vec![Action::Constant(0.), Action::WriteAngle, Action::Jet, Action::FirePayload],
                        ip: 4,
                        ..default()
                    },
                    Memory {
                        capacity: 30,
                        list: vec![],
                    },
                )).id(),
                commands.spawn((
                    InterpreterState {
                        actions: vec![Action::Constant(PI), Action::WriteAngle, Action::Jet, Action::FirePayload],
                        ip: 4,
                        ..default()
                    },
                    Memory {
                        capacity: 30,
                        list: vec![],
                    },
                )).id(),
                commands.spawn((
                    InterpreterState {
                        actions: vec![Action::Constant(-PI/2.), Action::WriteAngle, Action::Jet, Action::FirePayload],
                        ip: 4,
                        ..default()
                    },
                    Memory {
                        capacity: 30,
                        list: vec![],
                    },
                )).id(),
                commands.spawn((
                    InterpreterState {
                        actions: vec![Action::CursorAngle, Action::WriteAngle, Action::FirePayload, Action::Firebolt],
                        ip: 3,
                        ..default()
                    },
                    Memory {
                        capacity: 30,
                        list: vec![],
                    },
                )).id(),
            ];
            let player_id = commands.spawn((
                ManualControl(vec![
                    InputType::KeyCode(KeyCode::KeyA),
                    InputType::KeyCode(KeyCode::KeyD),
                    InputType::KeyCode(KeyCode::Space),
                    InputType::Mouse(MouseButton::Left),
                ]),
                CastEvents {
                    list: Vec::from(temp),
                    current: None,
                },
                Player,
                Health {
                    current: 100.,
                    max: 100.,
                    damage_mult: Damage {
                        fire: 1.,
                        kinetic: 1.,
                        cold: 1.,
                        poison: 1.,
                        emp: 0.,
                        decay: 1.,
                        disruptive: 10.,
                    }
                },
                Caster {
                    energy: 100.,
                    energy_max: 100.,
                },
                PayloadStorage {
                    capacity: 10,
                    list: vec![],
                },
                GlobalMemory(Memory {
                    capacity: 30,
                    list: vec![],
                }),
                (
                    InterpolateTransform::default(),
                    OldTransform(Transform::from_xyz(0., 0., 0.)),
                ),
                (
                    ReadMassProperties::default(),
                    Transform::from_xyz(0., 0., 0.),
                    Velocity::zero(),
                    Collider::cuboid(50., 100.),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Restitution::coefficient(0.1),
                ),
            )).add_children(&temp).id();
            commands.entity(player_id).insert(ProjectileGrace(player_id, 0));
            commands.spawn((
                Transform::from_xyz(0., -200., 0.),
                Collider::cuboid(500., 100.),
            ));
        },
        _ => {},
    }
}

fn lerp_camera_to_player(
    mut params: ParamSet<(
        Single<&mut Transform, With<MainCamera>>,
        Single<&Transform, With<Player>>,
    )>,
) {
    let player_pos = params.p1().into_inner().translation;
    let mut t = params.p0().into_inner();
    let diff = (player_pos - t.translation) * 0.05;
    t.translation += diff;
}