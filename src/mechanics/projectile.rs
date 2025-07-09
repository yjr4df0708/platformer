use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;
use crate::mechanics::effect::DealEffects;

use super::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
#[require(ActiveHooks)]
pub struct ProjectileGrace(pub Entity, pub u32);
//duration by ticks, not timer
//must be on the caster and its projectiles, with Entity pointing at the caster
//because projectiles spawn in the middle of most casters, some time is given until they may collide
//also prevents sibling projectiles from colliding for the duration

impl ProjectileGrace {
    pub fn system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut ProjectileGrace), With<Projectile>>,
    ) {
        for (entity, gracetime) in &mut query {
            if gracetime.1 > 0 {
                gracetime.into_inner().1 -= 1;
            } else {
                commands.entity(entity)
                    .remove::<ProjectileGrace>()
                    .remove::<ActiveHooks>()
                ;
            }
        }
    }
}

impl PartialEq for ProjectileGrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Component)]
pub struct Lifetime(pub u32);

impl Lifetime {
    pub fn system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Lifetime)>,
    ) {
        for (entity, mut lifetime) in &mut query {
            if lifetime.0 > 0 {
                lifetime.0 -= 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component)]
pub struct Piercing;

#[derive(Component)]
pub struct ContactDamage(Damage);

#[derive(Debug, Clone, Copy)]
pub enum ProjectileType {
    BasicBullet,
    Firebolt,
}

impl ProjectileType {
    fn damage(&self) -> Damage {
        match self {
            ProjectileType::BasicBullet => Damage { kinetic: 10., ..default() },
            ProjectileType::Firebolt => Damage { fire: 20., kinetic: 5., ..default() },
        }
    }
    pub fn create_entity(&self, mut commands: Commands, entity: Entity, angle: f32, payload: &Payload, transform: Transform, vel_opt: Option<&Velocity>) {
        let mut new_transform = transform;
        new_transform.rotation = new_transform.rotation.mul_quat(Quat::from_rotation_z(angle));
        let projectile_entity = commands.spawn((
            RigidBody::Dynamic,
            Lifetime(300),
            Ccd::enabled(),
            ProjectileGrace(entity, 30),//default, not tested, might have to be changed or vary by projectile type
            ActiveHooks::FILTER_CONTACT_PAIRS | ActiveHooks::FILTER_INTERSECTION_PAIR,
            Projectile,
            DealEffects(payload.effects.clone()),
            ContactDamage(self.damage().mul(&payload.damage_mult).add(&payload.damage_add.mul(&Damage::splat(1. / payload.projectiles.len() as f32)))),
            new_transform,
        )).id();
        let mut projectile_commands = commands.entity(projectile_entity);
        match self {
            ProjectileType::BasicBullet => projectile_commands.insert((
                Collider::ball(5.),//whatever bundle of components that a projectile inherently comes with
            )),
            ProjectileType::Firebolt => projectile_commands.insert((
                Collider::ball(10.),
            )),
        };
        let mut velocity = Velocity {
            angvel: 0.,
            linvel: Vec2::from_angle(angle) * match self {
                ProjectileType::BasicBullet => 2000.,
                ProjectileType::Firebolt => 1500.,
            },
        };
        if let Some(parent_velocity) = vel_opt {
            velocity.angvel += parent_velocity.angvel;
            velocity.linvel += parent_velocity.linvel;
        }
        projectile_commands.insert(velocity);
    }
}

#[derive(SystemParam)]
pub struct CollisionFilter<'w, 's> {
    tags: Query<'w, 's, &'static ProjectileGrace>,
}//maybe this should be moved out of projectiles later when other things need to use it

impl BevyPhysicsHooks for CollisionFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        match (self.tags.get(context.collider1()), self.tags.get(context.collider2())) {
            (Ok(ProjectileGrace(e1, _)), Ok(ProjectileGrace(e2, _))) => if e1 == e2 {
                    None
                } else {
                    Some(SolverFlags::COMPUTE_IMPULSES)
                },
            _ => Some(SolverFlags::COMPUTE_IMPULSES),
        }
    }
    fn filter_intersection_pair(&self, context: PairFilterContextView) -> bool {
        match (self.tags.get(context.collider1()), self.tags.get(context.collider2())) {
            (Ok(ProjectileGrace(e1, _)), Ok(ProjectileGrace(e2, _))) => e1 == e2,
            _ => false,
        }
    }
}