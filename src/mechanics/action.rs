use bevy_rapier2d::prelude::*;
use crate::state::CursorAngleRes;
use super::*;
use payload::Payload;
use projectile::Projectile;

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
    Jet,
    Constant(f32),
}

impl Action {
    pub fn system(
        cursor_angle: Res<CursorAngleRes>,
        mut query: Query<(
            &mut Caster,
            Option<&ReadMassProperties>,
            Option<&mut Velocity>,
            &mut PayloadStorage,
            &mut GlobalMemory,
            &mut Memory,
            &mut InterpreterState,
        )>,
    ) {
        for (caster, read_mass_opt, vel_opt, payloads, global_memory, memory, state) in &mut query {
            let (_caster, payloads,global_memory, memory, state) =
                (caster.into_inner(), payloads.into_inner(), global_memory.into_inner(), memory.into_inner(), state.into_inner());
            let action = state.tick();
            /*if let Action::NoOp = action {} else {
                println!("        running {:?}", action);
                println!("state: {:?}", state);
            }*/
            match action {
                Action::NoOp => (),
                Action::SwapRegisters => {
                    let temp = state.address;
                    state.address = state.register as usize;
                    state.register = temp as f32;
                },
                Action::SwapData => {
                    let temp = state.register;
                    let addr = state.address;
                    let res = memory.get(addr);
                    if let Ok(value) = res {
                        state.register = value;
                        memory.set(addr, temp).unwrap();
                    } else {//in case of out of bounds error
                        state.register = 0.;
                    }
                },
                Action::SwapDataGlobal => {
                    let temp = state.register;
                    let addr = state.address;
                    let res = global_memory.0.get(addr);
                    if let Ok(value) = res {
                        state.register = value;
                        global_memory.0.set(addr, temp).unwrap();
                    } else {
                        state.register = 0.;
                    }
                },
                Action::SwapPayload => {
                    let temp = state.payload.clone();
                    let addr = state.address;
                    let res = payloads.get(addr);
                    if let Ok(payload) = res {
                        state.payload = payload;
                        payloads.set(addr, temp).unwrap();
                    } else {
                        state.payload = Payload::default();//just delete the current payload?
                        //the user is trying to store it into a slot that doesn't exist
                    }
                },
                Action::WriteAngle => {
                    let temp = state.register.rem_euclid(2. * PI);
                    state.angle = temp;
                },
                Action::CursorAngle => {
                    state.register = if let Some(angle) = cursor_angle.0 {
                        angle
                    } else {
                        0.
                    };
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
                    //induce recoil
                    if let (Some(read_mass), Some(velocity)) = (read_mass_opt, vel_opt) {
                        let recoil = -state.payload.recoil * Vec2::from_angle(state.angle) / read_mass.get().mass;
                        println!("applying {} of recoil with original recoil {} and mass {}", recoil.length(), state.payload.recoil, read_mass.get().mass);
                        velocity.into_inner().linvel += recoil;
                    }
                    //destroy payload
                    state.payload = Payload::default();
                },
                Action::BasicBullet => {
                    //maybe cost mana, if not, remove &mut Caster
                    let state = state;
                    state.payload = state.payload.add(&Payload {
                        projectiles: vec![Projectile::BasicBullet],
                        ..default()
                    });
                    state.delay += 15;//quarter-second delay for adding a basic bullet projectile
                },
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