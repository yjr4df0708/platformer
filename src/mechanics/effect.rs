use super::*;
use bevy_rapier2d::prelude::*;
use damage::Damage;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectSystemSet;

pub fn plugin(app: &mut App) {
    app
        .add_systems(FixedUpdate, (
            Stun::system,
            ConsistentDamage::system,
            DamageFalloff::system,
            Weightless::system,
        ).chain().in_set(EffectSystemSet))
    ;
}

#[derive(Debug, Clone, Copy)]
pub enum EffectTypes {
    Stun(Stun),
    ConsistentDamage(ConsistentDamage),
    DamageFalloff(DamageFalloff),
    Weightless(Weightless),
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Stun {
    duration: u32
}

impl Stun {
    fn system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Stun)>,
    ) {
        for (entity, stun) in &mut query {
            if stun.duration > 0 {
                stun.into_inner().duration -= 1;
            } else {
                commands.entity(entity).remove::<Stun>();
            }
        }
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct ConsistentDamage {
    duration: u32,
    damage: Damage,
}

impl ConsistentDamage {
    fn system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Health, &mut ConsistentDamage)>,
    ) {
        for (entity, health, consistent_damage) in &mut query {
            if consistent_damage.duration > 0 {
                health.into_inner().apply_damage(&consistent_damage.damage);
                consistent_damage.into_inner().duration -= 1;
            } else {
                commands.entity(entity).remove::<ConsistentDamage>();
            }
        }
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct DamageFalloff {
    duration: u32,
    time: u32,
    damage: Damage,
}

impl DamageFalloff {
    pub fn system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Health, &mut DamageFalloff)>,
    ) {
        for (entity, health, damage_falloff) in &mut query {
            if damage_falloff.time > 0 {
                health.into_inner().apply_damage(&damage_falloff.damage.mul_scalar(damage_falloff.time as f32 / damage_falloff.duration as f32));
                damage_falloff.into_inner().time -= 1;
            } else {
                commands.entity(entity).remove::<DamageFalloff>();
            }
        }
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Weightless {
    duration: u32,
    temp: Option<GravityScale>,
}

impl Weightless {
    pub fn system(
        mut commands: Commands,
        mut query: Query<(Entity, Option<&mut GravityScale>, &mut Weightless)>,
    ) {
        for (entity, gravity_scale_opt, weightless) in &mut query {
            if weightless.duration > 0 {
                let weightless = weightless.into_inner();
                if let Some(gravity_scale) = gravity_scale_opt {
                    weightless.temp = Some(gravity_scale.clone());
                    commands.entity(entity).remove::<GravityScale>();
                }
                weightless.duration -= 1;
            } else {
                //return GravityScale from temp
                if let Some(gravity_scale) = weightless.temp {
                    commands.entity(entity).insert(gravity_scale);
                } else {
                    commands.entity(entity).remove::<GravityScale>();
                }
                //remove self
                commands.entity(entity).remove::<Weightless>();
            }
        }
    }
}