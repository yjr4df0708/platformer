use super::*;
use damage::Damage;
use effect::*;
use projectile::ProjectileType;

#[derive(Debug, Clone)]
pub struct Payload {
    pub damage_add: Damage,
    pub damage_mult: Damage,
    pub recoil: f32,
    pub speed_add: f32,
    pub projectiles: Vec<ProjectileType>,
    pub effects: Vec<EffectTypes>,
}

impl Payload {
    pub fn add(&self, rhs: &Self) -> Self {
        let mut temp = Payload { 
            damage_add: self.damage_add.add(&rhs.damage_add),
            damage_mult: self.damage_mult.add(&rhs.damage_mult),
            recoil: self.recoil + rhs.recoil,
            speed_add: self.speed_add + rhs.speed_add,
            projectiles: self.projectiles.clone(),
            effects: self.effects.clone(),
        };
        temp.projectiles.extend(&rhs.projectiles);
        temp.effects.extend(&rhs.effects);
        temp
    }
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            damage_add: Damage::default(),
            damage_mult: Damage::splat(1.),
            recoil: 0.,
            speed_add: 0.,
            projectiles: vec![],
            effects: vec![],
        }
    }
}