use bevy::prelude::*;
use crate::{
    cleanup_system,
    ui::*,
    state::prelude::*,
    GameState,
};
use super::{
    MenuState, 
    MenuEntity,
};

#[derive(Component)]
#[require(MenuEntity)]
struct PlayMenuEntity;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MenuState::PlayMenu), on_enter)
        .add_systems(Update, ui_input.run_if(in_state(MenuState::PlayMenu)))
        .add_systems(OnExit(MenuState::PlayMenu), cleanup_system::<PlayMenuEntity>)
    ;
}

#[derive(Component, Clone, Copy)]
enum ButtonId{
    Campaign,
    Endless,
    Testing,
    Back,
}

impl std::fmt::Display for ButtonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ButtonId::Campaign => "Campaign",
            ButtonId::Endless => "Endless",
            ButtonId::Testing => "Testing",
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
            row_gap: Val::Vh(10.),
            ..default_node!()
        },
        BackgroundColor(UI_BACKGROUND_COLOR),
        PlayMenuEntity,
    )).with_children(|builder| {
        for i in [ButtonId::Campaign, ButtonId::Endless, ButtonId::Testing, ButtonId::Back] {
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
                i,
            )).with_child((
                Text::new(format!("{}", i)),
                TextColor::BLACK,
            ));
        }
    });
}

fn ui_input(
    query: Query<(&Interaction, &ButtonId), Changed<Interaction>>,
    mut state: ResMut<GameMode>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, name) in &query {
        match (interaction, name) {
            (Interaction::Pressed, ButtonId::Campaign) => {
                *state = GameMode::Campaign;
                menu_state.set(MenuState::Disabled);
                game_state.set(GameState::Loading);
            },
            (Interaction::Pressed, ButtonId::Endless) => {
                *state = GameMode::Endless;
                menu_state.set(MenuState::Disabled);
                game_state.set(GameState::Loading);
            },
            (Interaction::Pressed, ButtonId::Testing) => {
                *state = GameMode::Testing;
                menu_state.set(MenuState::Disabled);
                game_state.set(GameState::Loading);
            },
            (Interaction::Pressed, ButtonId::Back) => {
                menu_state.set(MenuState::MainMenu);
            },
            (_, _) => {},
        }
    }
}