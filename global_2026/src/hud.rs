use bevy::prelude::*;

#[derive(Component)]
pub struct ExitButton;

pub fn hud_setup(mut commands: Commands) {
    // Version text (bottom-right)
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

    // Exit button (just above the version text)
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
}

pub fn exit_button_system(
    mut commands: Commands,
    button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in &button_query {
        if *interaction == Interaction::Pressed {
            // Send the exit message via commands
            commands.write_message(AppExit::Success);
        }
    }
}