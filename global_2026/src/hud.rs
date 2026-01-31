use bevy::prelude::*;

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
}
pub fn hud_update() {
    //    println!("item update")
}
pub fn hud_fixed_update() {
    //    println!("item fixed update")
}
