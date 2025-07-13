use bevy::{
    math::ops::powf, prelude::*
};

pub use std::f32::consts::PI;

/* cast events are handled as child entities
 * mainly because actions are handled as ZST markers
 * and a caster can have multiple cast events
 * (actually considering having some actions with storage, not sure though,
 * it's kind of hard to make numbers out of nothing, we should probably
 * have some kind of "mov eax, (immediate)" instruction but idk about ui)
 * 
 * main entity:
 * - Option<Health>
 * - Caster (energy current, max)
 * - PayloadStorage
 * - GlobalMemory
 * - Some kind of AI or controller triggering the castevents,
 *   with entity references
 * - some effects
 * |>child entity (CastEvent1):
 * | - Memory
 * | - InterpreterState
 * so a query can find the child entity 
 * 
 * the interpreter, each frame, will run up to 10 actions
 * or until an action adds delay
 */

pub mod action;
pub mod payload;
pub mod damage;
pub mod effect;
pub mod projectile;

use action::Action;
use payload::Payload;
use damage::Damage;

use crate::MainCamera;

#[derive(Debug)]
enum MechanicsError {
    AddrOutOfBounds,
}

#[derive(Component)]
pub struct Caster {
    pub energy: f32,
    pub energy_max: f32,
}

#[derive(Component, Default)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub damage_mult: Damage,
}
//should Health contain its own resistances or should it be its own component?

impl Health {
    fn apply_damage(&mut self, damage: &Damage) {
        let x = self.current - self.damage_mult.mul(damage).sum();
        if x > self.max {
            self.current = self.max;
        } else {
            self.current = x;
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct Memory {
    pub capacity: usize,//should usually be within the range of like 20
    pub list: Vec<f32>,//always initialize with 0 to match list.len() to capacity
}

#[derive(Component)]
pub struct GlobalMemory(pub Memory);

impl Memory {//only the same getter and setter, same as PayloadStorage and could be turned into an index trait
    fn get(&mut self, index: usize) -> Result<f32, MechanicsError> {
        if index < self.capacity {
            if index < self.list.len() {
                Ok(self.list[index])
            } else {
                for _ in self.list.len()..=index {
                    self.list.push(0.);
                }
                Ok(0.)
            }
        } else {
            Err(MechanicsError::AddrOutOfBounds)
        }
    }
    fn set(&mut self, index: usize, value: f32) -> Result<(), MechanicsError> {
        if index < self.capacity as usize {
            Err(MechanicsError::AddrOutOfBounds)
        } else {
            if index >= self.list.len() {
                for i in self.list.len()..index {
                    self.list[i] = 0.;
                }
            }
            self.list[index] = value;
            Ok(())
        }
    }
}

#[derive(Component)]
pub struct PayloadStorage {
    pub capacity: usize,
    pub list: Vec<Payload>,
}

impl PayloadStorage {//only the same getter and setter, same as Memory and could be turned into an index trait
    fn get(&mut self, index: usize) -> Result<Payload, MechanicsError> {
        if index < self.capacity {
            if index < self.list.len() {
                Ok(self.list[index].clone())
            } else {
                for _ in self.list.len()..=index {
                    self.list.push(Payload::default());
                }
                Ok(Payload::default())
            }
        } else {
            Err(MechanicsError::AddrOutOfBounds)
        }
    }
    fn set(&mut self, index: usize, value: Payload) -> Result<(), MechanicsError> {
        if index < self.capacity as usize {
            Err(MechanicsError::AddrOutOfBounds)
        } else {
            if index >= self.list.len() {
                for i in self.list.len()..index {
                    self.list[i] = Payload::default();
                }
            }
            self.list[index] = value;
            Ok(())
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct InterpreterState {
    pub register: f32,
    pub address: usize,
    pub ip: usize,//instruction pointer, for internal use or jump instructions
    pub angle: f32,
    pub payload: Payload,
    pub delay: u32,//number of ticks to wait until next run
    pub capacity: u32,
    pub actions: Vec<Action>,
}

impl InterpreterState {
    fn tick(&mut self) -> Action {
        if self.ip < self.actions.len() && self.delay == 0 {
            self.ip += 1;
            self.actions[self.ip - 1]
        } else {
            Action::default()
        }
    }
}

struct TickInterpreterDelay(Entity);

impl Command<Result> for TickInterpreterDelay {
    fn apply(self, world: &mut World) -> Result {
        if let Some(mut state) = world.get_entity_mut(self.0)?.get_mut::<InterpreterState>() {
            if state.delay > 0 {
                state.delay -= 1;
            }
        }
        Ok(())
    }
}

#[derive(Component)]
pub struct CastFocus(pub Option<f32>);

#[derive(Component)]
pub struct CastEvents {
    pub list: Vec<Entity>,//all children of the current entity
    pub current: Option<usize>,
}

struct MoveCastState(Entity, Entity);

impl Command<Result> for MoveCastState {
    fn apply(self, world: &mut World) -> Result {
        let mut state = InterpreterState::default();
        let mut state_set = false;
        let mut memory = Memory { capacity: 0, list: vec![] };
        let mut memory_set = false;
        {
            let from = world.get_entity(self.1)?;
            if let Some(from_state) = from.get::<InterpreterState>() {
                state = from_state.clone();
                state_set = true;
            }
            if let Some(from_memory) = from.get::<Memory>() {
                memory = from_memory.clone();
                memory_set = true;
            }
        }
        {
            let mut to = world.get_entity_mut(self.0)?;
            if state_set {
                to.insert(state);
            }
            if memory_set {
                to.insert(memory);
            }
        }
        {
            let mut from = world.get_entity_mut(self.1)?;
            if state_set {
                from.remove::<InterpreterState>();
            }
            if memory_set {
                from.remove::<Memory>();
            }
        }
        Ok(())
    }
}

impl CastEvents {
    fn system(
        In(i): In<Option<usize>>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut CastEvents)>,
    ) -> bool {
        let mut any_exist = false;
        for (entity, mut cast_events) in &mut query {
            //move any existing cast event components into cast_events.list[cast_events.current]
            if let Some(n) = cast_events.current {
                commands.queue(MoveCastState(cast_events.list[n], entity));
                commands.queue(TickInterpreterDelay(cast_events.list[n]));
            }
            cast_events.current = i;
            if let Some(i) = i {
                if i < cast_events.list.len() {
                    //move existing cast event components into main entity
                    commands.queue(MoveCastState(entity, cast_events.list[i]));
                    any_exist = true;
                } else {
                    cast_events.current = None;
                }
            }
        }
        any_exist
    }
}

pub enum InputType {
    KeyCode(KeyCode),
    Mouse(MouseButton),
}

#[derive(Component)]
pub struct ManualControl(pub Vec<InputType>);

impl ManualControl {
    pub fn system(
        mut commands: Commands,
        mouse: Res<ButtonInput<MouseButton>>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut set: ParamSet<(
            Query<(&CastEvents, &ManualControl)>,
            Single<(&Transform, &mut CastFocus), With<Player>>,
            Single<&Window>,
            Single<(&Camera, &GlobalTransform), With<MainCamera>>,
        )>,
    ) {
        for (cast_events, control) in &set.p0() {
            for i in 0..cast_events.list.len().min(control.0.len()) {
                match control.0[i] {
                    InputType::KeyCode(keycode) => {
                        if keyboard.pressed(keycode) {
                            commands.entity(cast_events.list[i]).entry::<InterpreterState>().and_modify(|mut state| {
                                if state.ip == state.actions.len() && state.delay == 0 {
                                    state.ip = 0;
                                }
                            });
                        }
                    },
                    InputType::Mouse(mouse_button) => {
                        if mouse.pressed(mouse_button) {
                            commands.entity(cast_events.list[i]).entry::<InterpreterState>().and_modify(|mut state| {
                                if state.ip == state.actions.len() && state.delay == 0 {
                                    state.ip = 0;
                                }
                            });
                        }
                    },
                }
            }
        }
        if let Some(world_position) = set.p2().cursor_position() {
            let (camera, camera_transform) = *set.p3();
            if let Ok(pos) = camera.viewport_to_world_2d(camera_transform, world_position) {
                set.p1().1.0 = Some((pos - set.p1().0.translation.truncate()).to_angle());
                return;
            }
        }
        set.p1().1.0 = None;
    }
}

pub fn caster_system(
    world: &mut World,
) {//I am concerned
    let mut i = 0;
    while world.run_system_cached_with(CastEvents::system, Some(i)).unwrap() {
        for _ in 0..10 {
            world.run_system_cached(Action::system).unwrap();
        }
        i += 1;
    }
    world.run_system_cached_with(CastEvents::system, None).unwrap();
}