use super::*;
use damage::Damage;
use effect::*;
use projectile::Projectile;

#[derive(Debug, Clone)]
pub struct Payload {
    pub damage_add: Damage,
    pub damage_mult: Damage,
    pub recoil: f32,
    pub projectiles: Vec<Projectile>,
    pub effects: Vec<EffectTypes>,
}

impl Payload {
    pub fn add(&self, rhs: &Self) -> Self {
        let mut temp = Payload { 
            damage_add: self.damage_add.add(&rhs.damage_add),
            damage_mult: self.damage_mult.add(&rhs.damage_mult),
            recoil: self.recoil + rhs.recoil,
            projectiles: self.projectiles.clone(),
            effects: self.effects.clone(),
        };
        temp.projectiles.extend(&rhs.projectiles);
        temp.effects.extend(&rhs.effects);
        println!("{:?} + {:?} = {:?}", self, rhs, temp);
        temp
    }
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            damage_add: Damage::default(),
            damage_mult: Damage::splat(1.),
            recoil: 0.,
            projectiles: vec![],
            effects: vec![],
        }
    }
}