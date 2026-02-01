
use bevy::prelude::*;
use crate::GameState;
use crate::unmasked::UnmaskedScore;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct RestartButton;
#[derive(Component)]
pub struct ScoreText;

pub fn gameover_setup(mut commands: Commands) {
    commands.spawn((
        Text::new("Game Over"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(500.0),
            right: Val::Px(500.0),
            ..default()
        },
    ));
}
