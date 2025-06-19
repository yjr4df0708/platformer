use super::*;
use payload::Payload;
use projectile::Projectile;

pub enum Action {
    SwapRegisters,
    SwapData,
    SwapDataGlobal,
    SwapPayload,
    Add,
    Sub,
    Mult,
    Div,
    Power,
    BasicBullet,
}

impl Action {
    pub fn run(&self, state: &mut InterpreterState, memory: &mut Memory, global_memory: &mut Memory, payloads: &mut PayloadStorage) {
        match self {
            Action::SwapRegisters => {
                let temp = state.address;
                state.address = state.register as u32;
                state.register = temp as f32;
            },
            Action::SwapData => {
                let temp = state.register;
                let res = memory.get(state.address as usize);
                if let Ok(value) = res {
                    state.register = value;
                    memory.set(state.address as usize, temp).unwrap();
                } else {//in case of out of bounds error
                    state.register = 0.;
                }
            },
            Action::SwapDataGlobal => {
                let temp = state.register;
                let res = global_memory.get(state.address as usize);
                if let Ok(value) = res {
                    state.register = value;
                    global_memory.set(state.address as usize, temp).unwrap();
                } else {
                    state.register = 0.;
                }
            },
            Action::SwapPayload => {
                let temp = state.payload.clone();
                let res = payloads.get(state.address as usize);
                if let Ok(payload) = res {
                    state.payload = payload;
                    payloads.set(state.address as usize, temp).unwrap();
                } else {
                    state.payload = Payload::default();//just delete the current payload?
                    //or should we do something else, like fire it?
                }
            },
            Action::Add => {
                if let Ok(value) = memory.get(state.address as usize) {
                    state.register += value;
                }
            },
            Action::Sub => {
                if let Ok(value) = memory.get(state.address as usize) {
                    state.register -= value;
                }
            },
            Action::Mult => {
                if let Ok(value) = memory.get(state.address as usize) {
                    state.register *= value;
                }
            },
            Action::Div => {
                if let Ok(value) = memory.get(state.address as usize) {
                    state.register /= value;
                }
            },
            Action::Power => {
                if let Ok(value) = memory.get(state.address as usize) {
                    state.register = powf(state.register, value);
                }
            },
            Action::BasicBullet => {
                state.payload = state.payload.add(&Payload {
                    projectiles: vec![Projectile::BasicBullet],
                    ..default()
                });
                state.delay+=15;//quarter-second delay for adding a basic bullet projectile
            },
        }
    }
}