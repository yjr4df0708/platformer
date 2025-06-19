#[derive(Clone, Copy)]
pub struct Damage {
    pub fire: f32,
    pub kinetic: f32,
    pub cold: f32,
    pub poison: f32,
    pub emp: f32,
    pub decay: f32,
    pub disruptive: f32,//looks like a glitch, related enemies inflict this and are immune
}

impl Damage {
    pub fn add(&self, rhs: &Self) -> Self {
        Damage {
            fire: self.fire + rhs.fire,
            kinetic: self.kinetic + rhs.kinetic,
            cold: self.cold + rhs.cold,
            poison: self.poison + rhs.poison,
            emp: self.emp + rhs.emp,
            decay: self.decay + rhs.decay,
            disruptive: self.disruptive + rhs.disruptive,
        }
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        Damage {
            fire: self.fire * rhs.fire,
            kinetic: self.kinetic * rhs.kinetic,
            cold: self.cold * rhs.cold,
            poison: self.poison * rhs.poison,
            emp: self.emp * rhs.emp,
            decay: self.decay * rhs.decay,
            disruptive: self.disruptive * rhs.disruptive,
        }
    }
    pub fn splat(value: f32) -> Self {
        Damage { fire: value, kinetic: value, cold: value, poison: value, emp: value, decay: value, disruptive: value, }
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self::splat(0.)
    }
}