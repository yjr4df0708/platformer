use bevy::prelude::*;

use crate::{mechanics::Player, MainCamera};

//where global game state is defined and stored
//like game difficulty/mode, settings

pub fn plugin(app: &mut App) {
    app
        .insert_resource(GameMode::Campaign)
        .insert_resource(Volume(0.))
        .insert_resource(PlayerEntity(None))
        .insert_resource(MainCameraEntity(None))
        .insert_resource(CursorAngleRes(None))
        .insert_resource(ClosestEnemyAngle(None))
    ;
}

pub mod prelude {
    pub use super::{
        GameMode,
        Volume,
    };
}

#[derive(Resource)]
pub enum GameMode {
    Campaign,
    Endless,
    Testing,
}

#[derive(Resource)]
pub struct Volume(pub f32);

#[derive(Resource)]
pub struct PlayerEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct MainCameraEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct CursorAngleRes(pub Option<f32>);

impl CursorAngleRes {
    pub fn system(
        mut cursor_angle_res: ResMut<CursorAngleRes>,
        mut set: ParamSet<(
            Single<&Transform, With<Player>>,
            Single<&Window>,
            Single<(&Camera, &GlobalTransform), With<MainCamera>>,
        )>,
    ) {
        if let Some(world_position) = set.p1().cursor_position() {
            let (camera, camera_transform) = *set.p2();
            if let Ok(pos) = camera.viewport_to_world_2d(camera_transform, world_position) {
                cursor_angle_res.0 = Some((pos - set.p0().translation.truncate()).to_angle());
                return;
            }
        }
        cursor_angle_res.0 = None;
    }
}

#[derive(Resource)]
pub struct ClosestEnemyAngle(pub Option<f32>);