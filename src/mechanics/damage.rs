#[derive(Debug, Clone, Copy)]
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
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        Damage {
            fire: self.fire * rhs,
            kinetic: self.kinetic * rhs,
            cold: self.cold * rhs,
            poison: self.poison * rhs,
            emp: self.emp * rhs,
            decay: self.decay * rhs,
            disruptive: self.disruptive * rhs,
        }
    }
    pub fn splat(value: f32) -> Self {
        Damage { fire: value, kinetic: value, cold: value, poison: value, emp: value, decay: value, disruptive: value, }
    }
    pub fn sum(&self) -> f32 {
        self.fire + self.kinetic + self.cold + self.poison + self.emp + self.decay + self.disruptive
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self::splat(0.)
    }
}