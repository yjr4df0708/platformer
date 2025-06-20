use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState{
    #[default]
    Menu,
    Loading,
    Running,
}

mod menu;
mod loading;
mod running;

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>,) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Platformer".to_string(),
                canvas: Some("#platformer".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .init_state::<GameState>()
        .add_systems(Startup, startup)
        .add_systems(Update, ui::ui_background_colors_system)
        .add_plugins((state::plugin, menu::plugin, loading::plugin, running::plugin))
        .run();
}

pub mod ui;
pub mod state;
pub mod mechanics;