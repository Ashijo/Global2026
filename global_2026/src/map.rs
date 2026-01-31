use bevy::prelude::*;
use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;

pub fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn((
        Map,
        Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            image: asset_server.load("map.png"),
            ..default()
        },
        Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0),
    ));
}

pub fn map_update() {
    //println!("map update")
}

pub fn map_fixed_update() {
    //println!("map fixed update")
}

#[derive(Component)]
pub struct Map;
