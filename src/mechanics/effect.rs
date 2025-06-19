use super::*;
use damage::Damage;

#[derive(Clone, Copy)]
pub enum Effect {
    Stun(u32),//first is duration in ticks
    ConsistentDamage(u32, Damage),//second is damage per tick
    DamageFalloff(u32, Damage),//ticks until Damage dies off, Damage is proportional to time remaining
}