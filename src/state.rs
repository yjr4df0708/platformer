use bevy::{
    asset::AssetPath,
    prelude::*,
};

//where global game state is defined and stored
//like game difficulty/mode, settings

pub fn plugin(app: &mut App) {
    app
        .insert_resource(GameMode::Campaign)
        .insert_resource(SaveFile(None))
        .insert_resource(Volume(0.))
        .insert_resource(PlayerEntity(None))
        .insert_resource(MainCameraEntity(None))
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
pub struct SaveFile<'a>(pub Option<AssetPath<'a>>);

#[derive(Resource)]
pub struct Volume(pub f32);

#[derive(Resource)]
pub struct PlayerEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct MainCameraEntity(pub Option<Entity>);