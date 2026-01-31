use bevy::prelude::*;

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(25.)),
            image: asset_server.load("test.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
pub fn player_update() {
    println!("player update")
}
pub fn player_fixed_update() {
    println!("player fixed update")
}

#[derive(Component)]
struct Player;
