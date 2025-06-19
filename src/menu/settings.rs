use bevy::prelude::*;
use crate::{
    cleanup_system,
    ui::*,
};
use super::{
    MenuState, 
    MenuEntity
};

#[derive(Component)]
#[require(MenuEntity)]
struct SettingsEntity;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MenuState::Settings), on_enter)
        .add_systems(Update, ui_input.run_if(in_state(MenuState::Settings)))
        .add_systems(OnExit(MenuState::Settings), cleanup_system::<SettingsEntity>)
    ;
}

#[derive(Component, Clone, Copy)]
enum ButtonId{
    //,
    //,
    Back,
}

impl std::fmt::Display for ButtonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ButtonId::Back => "Back",
        })
    }
}

fn on_enter(
    mut commands: Commands,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default_node!()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        SettingsEntity,
    )).with_children(|builder| {
        builder.spawn((
            Node {
                width: Val::Vw(20.),
                ..default_node!()
            },
            UiBackgroundColors {
                normal: BackgroundColor(UI_FOREGROUND_COLOR),
                focus: BackgroundColor(UI_FOCUS_COLOR),
            },
            Button,
            ButtonId::Back,
        )).with_child((
            Text::new(format!("{}", ButtonId::Back)),
            TextColor::BLACK,
        ));
    });
}

fn ui_input(
    query: Query<(&Interaction, &ButtonId), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, name) in &query {
        match (interaction, name) {
            (Interaction::Pressed, ButtonId::Back) => {
                menu_state.set(MenuState::MainMenu);
            },
            (_, _) => {},
        }
    }
}