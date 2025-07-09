use bevy::prelude::*;

//not sure how but it still looks like shit

#[derive(Component, Default)]
#[require(Transform)]
pub struct OldTransform(pub Transform);

#[derive(Component, Default)]
#[require(OldTransform)]
pub struct InterpolateTransform(Transform);

//Before FixedUpdate (FixedFirst) copy Transform into OldTransform

//just before Extract (Last) we will copy Transform into InterpolateTransform
//and replace Transform with an interpolation approximation

//just after Extract (First) we restore the Transform from InterpolateTransform

fn update_old_transform_system(
    mut query: Query<(&Transform, &mut OldTransform)>,
) {
    for (transform, mut old_transform) in &mut query {
        old_transform.0 = *transform;
    }
}

pub fn interpolate_transform_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &OldTransform, &mut InterpolateTransform)>,
) {
    let overstep = time.overstep_fraction();
    for (mut transform, old_transform, mut interpolate_transform) in &mut query {
        let temp = interpolate_transform.0;
        interpolate_transform.0 = *transform;
        *transform = Transform::interpolate(&old_transform.0, &temp, overstep);
    }
}

fn interpolate_restore_system(
    mut query: Query<(&mut Transform, &InterpolateTransform)>,
) {
    for (mut transform, interpolate_transform) in &mut query {
        *transform = interpolate_transform.0;
    }
}

pub fn plugin(app: &mut App) {
    app
        .add_systems(FixedFirst, update_old_transform_system)
        .add_systems(Last, interpolate_transform_system)
        .add_systems(First, interpolate_restore_system)
    ;
}