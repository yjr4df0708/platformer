use bevy::prelude::*;

//where global game state is defined and stored
//like game difficulty/mode, settings

pub fn plugin(app: &mut App) {
    app
        .insert_resource(GameMode::Campaign)
        .insert_resource(Level(0))
        .insert_resource(Volume(0.))
    ;
}

pub mod prelude {
    pub use super::{
        GameMode,
        Level,
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
pub struct Level(u32);

#[derive(Resource)]
pub struct Volume(f32);