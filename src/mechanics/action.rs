use bevy_rapier2d::prelude::*;
use crate::state::CursorAngleRes;
use super::*;
use payload::Payload;
use projectile::ProjectileType;

#[derive(Component, Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    NoOp,
    SwapRegisters,
    SwapData,
    SwapDataGlobal,
    SwapPayload,
    WriteAngle,
    CursorAngle,
    Add,
    Sub,
    Mult,
    Div,
    Power,
    FirePayload,
    BasicBullet,
    Firebolt,
    Jet,
    Constant(f32),
}

impl Action {
    pub fn system(
        mut commands: Commands,
        cursor_angle: Res<CursorAngleRes>,
        mut query: Query<(
            Entity,
            &mut Caster,
            Option<&ReadMassProperties>,
            Option<&mut Velocity>,
            &Transform,//maybe we can add teleport later, also this assumes no casters are child entities
            &mut PayloadStorage,
            &mut GlobalMemory,
            &mut Memory,
            &mut InterpreterState,
        )>,
    ) {
        for (entity, mut _caster, read_mass_opt, vel_opt, transform, mut payloads, mut global_memory, mut memory, mut state) in &mut query {
            match state.tick() {
                Action::NoOp => (),
                Action::SwapRegisters => {
                    let temp = state.address;
                    state.address = state.register as usize;
                    state.register = temp as f32;
                },
                Action::SwapData => {
                    if let Ok(value) = memory.get(state.address) {
                        let temp = state.register;
                        state.register = value;
                        memory.set(state.address, temp).unwrap();
                    } else {//in case of out of bounds error
                        state.register = 0.;
                    }
                },
                Action::SwapDataGlobal => {
                    if let Ok(value) = global_memory.0.get(state.address) {
                        let temp = state.register;
                        state.register = value;
                        global_memory.0.set(state.address, temp).unwrap();
                    } else {
                        state.register = 0.;
                    }
                },
                Action::SwapPayload => {
                    if let Ok(payload) = payloads.get(state.address) {
                        let temp = state.payload.clone();
                        state.payload = payload;
                        payloads.set(state.address, temp).unwrap();
                    } else {
                        state.payload = Payload::default();//just delete the current payload?
                        //the user is trying to store it into a slot that doesn't exist
                    }
                },
                Action::WriteAngle => {
                    state.angle = state.register.rem_euclid(2. * PI);
                },
                Action::CursorAngle => {
                    state.register = cursor_angle.0.unwrap_or(0.);
                },
                Action::Add => {
                    if let Ok(value) = memory.get(state.address) {
                        state.register += value;
                    }
                },
                Action::Sub => {
                    if let Ok(value) = memory.get(state.address) {
                        state.register -= value;
                    }
                },
                Action::Mult => {
                    if let Ok(value) = memory.get(state.address) {
                        state.register *= value;
                    }
                },
                Action::Div => {
                    if let Ok(value) = memory.get(state.address) {
                        state.register /= value;
                    }
                },
                Action::Power => {
                    if let Ok(value) = memory.get(state.address) {
                        state.register = powf(state.register, value);
                    }
                },
                Action::FirePayload => {
                    //todo check if there is enough mana before firing, drain mana...
                    //should mana drain happen when building or firing payloads?
                    //if it's on building it makes more sense, and forces players to reorder their actions, more interesting
                    //spawn projectiles
                    for projectile in &state.payload.projectiles {
                        projectile.create_entity(commands.reborrow(), entity, state.angle, &state.payload, *transform, vel_opt.as_deref());
                    }
                    //induce recoil
                    if let (Some(read_mass), Some(mut velocity)) = (read_mass_opt, vel_opt) {
                        velocity.linvel -= state.payload.recoil * Vec2::from_angle(state.angle) / read_mass.get().mass;
                    }
                    //destroy payload
                    state.payload = Payload::default();
                },
                Action::BasicBullet => {
                    //maybe cost mana, if not, remove &mut Caster
                    state.payload = state.payload.add(&Payload {
                        projectiles: vec![ProjectileType::BasicBullet],
                        ..default()
                    });
                    state.delay += 15;//quarter-second delay for adding a basic bullet projectile
                },
                Action::Firebolt => {
                    state.payload = state.payload.add(&Payload {
                        projectiles: vec![ProjectileType::Firebolt],
                        ..default()
                    });
                    state.delay += 20;
                }
                Action::Jet => {
                    state.payload = state.payload.add(&Payload {
                        recoil: 1000000.,
                        ..default()
                    });
                },
                Action::Constant(constant) => {
                    state.register = constant;
                }
            }
        }
    }
}