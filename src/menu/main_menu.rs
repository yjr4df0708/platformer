use bevy::prelude::*;
use crate::{
    cleanup_system,
    ui::*,
};
use super::{
    MenuState,
    MenuEntity,
};

#[derive(Component)]
#[require(MenuEntity)]
struct MainMenuEntity;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MenuState::MainMenu), on_enter)
        .add_systems(Update, ui_input.run_if(in_state(MenuState::MainMenu)))
        .add_systems(OnExit(MenuState::MainMenu), cleanup_system::<MainMenuEntity>)
    ;
}

#[derive(Component, Clone, Copy)]
enum ButtonId{
    Play,
    Settings,
    Quit,
}

impl std::fmt::Display for ButtonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ButtonId::Play => "Play",
            ButtonId::Settings => "Settings",
            ButtonId::Quit => "Quit",
        })
    }
}

fn on_enter(
    mut commands: Commands
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            row_gap: Val::Vh(10.),
            ..(default_node!())
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        MainMenuEntity,
    )).with_children(|builder| {
        for i in [ButtonId::Play, ButtonId::Settings, ButtonId::Quit] {
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
    mut app_exit_event: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, name) in &query {
        match (interaction, name) {
            (Interaction::Pressed, ButtonId::Play) => {
                menu_state.set(MenuState::PlayMenu);//choose what to play
            },
            (Interaction::Pressed, ButtonId::Settings) => {
                menu_state.set(MenuState::Settings)
            },
            (Interaction::Pressed, ButtonId::Quit) => {
                app_exit_event.write(AppExit::Success);
            },
            (_, _) => {},
        }
    }
}