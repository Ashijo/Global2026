use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
struct Mask;

#[derive(Resource)]
struct MaskSpawner {
    timer: Timer,
    max_masks: usize,
    min: Vec2,
    max: Vec2,
    rng: ThreadRng, // random number generator
}

pub fn mask_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Mask,
        Sprite {
            custom_size: Some(Vec2::splat(50.)),
            image: asset_server.load("img/mask.png"),
            ..default()
        },
        Transform::from_xyz(250.0, 250.0, 1.0),
    ));
}

pub fn mask_fixed_upgrade() {

}