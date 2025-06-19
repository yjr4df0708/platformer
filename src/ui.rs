use bevy::prelude::*;

macro_rules! default_node {
    () => {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    };
}
pub(crate) use default_node;

pub const UI_BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const UI_FOREGROUND_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const UI_FOCUS_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Component, Default)]
pub struct UiBackgroundColors{
    pub normal: BackgroundColor,
    pub focus: BackgroundColor,
}

pub fn ui_background_colors_system(
    mut query: Query<(&Interaction, &UiBackgroundColors, &mut BackgroundColor), Changed<Interaction>>,
) {
    for i in &mut query {
        match i {
            (Interaction::None, colors, mut bg) => {
                *bg.as_mut() = colors.normal;
            },
            (_, colors, mut bg) => {
                *bg.as_mut() = colors.focus;
            },
        }
    }
}