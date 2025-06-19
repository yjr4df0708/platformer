use bevy::{
    math::ops::powf,
    prelude::*
};

/* events reference attached keycodes
 * events contain arrays/vectors of actions
 * actions have certain effects on interpreter state when run
 * maybe actions can either be implemented as a trait with a run function
 * or as an enum that just `matches` itself
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

#[derive(Debug)]
enum MechanicsError {
    AddrOutOfBounds,
}

#[derive(Component)]
pub struct Memory {
    capacity: u32,//should usually be within the range of like 20
    list: Vec<f32>,//always initialize with 0 to match list.len() to capacity
}

impl Memory {//only the same getter and setter, same as PayloadStorage and could be turned into an index trait
    fn get(&self, index: usize) -> Result<f32, MechanicsError> {
        if index < self.list.len() {
            Ok(self.list[index])
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
    capacity: u32,
    list: Vec<Payload>,
}

impl PayloadStorage {//only the same getter and setter, same as PayloadStorage and could be turned into an index trait
    fn get(&self, index: usize) -> Result<Payload, MechanicsError> {
        if index < self.list.len() {
            Ok(self.list[index].clone())
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

#[derive(Component)]
pub struct Event {
    capacity: u32,
    action_list: Vec<Action>,
    state: InterpreterState,
    memory: Memory,
}

pub struct InterpreterState {
    register: f32,
    address: u32,
    ip: u32,//instruction pointer, for internal use or jump instructions
    angle: f32,
    payload: Payload,
    delay: u32,//number of ticks to wait until next run
}