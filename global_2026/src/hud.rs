use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct RestartButton;

pub fn hud_setup(mut commands: Commands) {
    commands.spawn((
        Text::new("Bomberdude 0.0.1"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));

    /*
    commands.spawn((
        Text::new("Unmasked:0"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));


    commands.spawn((
        Text::new("Truth Bombs:5"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
    */
    
    // Exit Button
    commands
        .spawn((
            Button,
            ExitButton,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Exit"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE.into()),
            ));
        });

    // Restart Button
    commands
        .spawn((
            Button,
            RestartButton,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(70.0),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Restart"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE.into()),
            ));
        });
}

pub fn hud_update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, Option<&RestartButton>, Option<&ExitButton>), Changed<Interaction>>,
) {
    for (interaction, restart, exit) in &query {
        if *interaction == Interaction::Pressed {
            if exit.is_some() {
                commands.write_message(AppExit::Success);
            }
            if restart.is_some() {
                next_state.set(GameState::Restart);
            }
        }
    }
}