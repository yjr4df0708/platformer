use super::*;
use damage::Damage;
use effect::Effect;
use projectile::Projectile;

#[derive(Clone)]
pub struct Payload {
    pub damage_add: Damage,
    pub damage_mult: Damage,
    pub projectiles: Vec<Projectile>,
    pub effects: Vec<Effect>,
}

impl Payload {
    pub fn add(&self, rhs: &Self) -> Self {
        let mut temp = Payload { 
            damage_add: self.damage_add.add(&rhs.damage_add),
            damage_mult: self.damage_mult.add(&rhs.damage_mult),
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
            projectiles: vec![],
            effects: vec![],
        }
    }
}