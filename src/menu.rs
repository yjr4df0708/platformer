use bevy::prelude::*;
use crate::{cleanup_system, GameState};

#[derive(Component, Default)]
struct MenuEntity;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MenuState {
    #[default]
    Disabled,
    MainMenu,
    PlayMenu,
    Settings,
}

mod main_menu;
mod play_menu;
mod settings;

pub fn plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), on_enter)
        .add_systems(OnExit(GameState::Menu), cleanup_system::<MenuEntity>)
        .add_plugins((main_menu::plugin, play_menu::plugin, settings::plugin))
    ;
}

fn on_enter(
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    menu_state.set(MenuState::MainMenu);
}