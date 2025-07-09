use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//if this looks to bad because of inaccuracy I might just switch to interpolation

#[derive(Component, Default)]
#[require(Transform, Velocity)]
pub struct ExtrapolateTransform(Transform);
//just before Extract (Last) we will move Dynamic body Transforms into this,
//then make up a new Transform based on extrapolation
//just after Extract (First) we will restore the Transform from this

pub fn extrapolate_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&Velocity, &mut Transform, &mut ExtrapolateTransform)>,
) {
    let dt = time.overstep_fraction();
    let delta_secs = time.delta_secs();
    for (velocity, mut transform, mut extrapolate) in &mut query {
        extrapolate.0 = *transform;
        let diff = (velocity.linvel * delta_secs * dt).extend(0.);
        transform.translation += diff;
        transform.rotate_z(velocity.angvel * dt);
        //should also rotate the translation around the center of mass, but that's for later
    }
}

fn restore_extrapolate_system(
    mut query: Query<(&mut Transform, &ExtrapolateTransform)>,
) {
    for (mut transform, extrapolate) in &mut query {
        *transform = extrapolate.0;
    }
}

pub fn plugin(app: &mut App) {
    app
        .add_systems(First, restore_extrapolate_system)
        .add_systems(Last, extrapolate_system)
    ;
}