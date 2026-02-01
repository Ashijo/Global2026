use bevy::prelude::*;
use rand::RngExt;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::collision::Hitbox;
use crate::level::LevelEntity;

const BORDER_SIZE: f32 = 100.0;

//rng = random number generation
#[derive(Component)]
pub struct Mask;

#[derive(Resource)]
pub struct MaskSpawner {
    timer: Timer,
    max_masks: usize,
    min: Vec2,
    max: Vec2,
}

pub fn mask_setup(mut commands: Commands) {
    commands.insert_resource(MaskSpawner {
        timer: Timer::from_seconds(15.0, TimerMode::Repeating),
        max_masks: 3,
        min: Vec2::new(BORDER_SIZE, BORDER_SIZE),
        max: Vec2::new(WINDOW_WIDTH - BORDER_SIZE, WINDOW_HEIGHT - BORDER_SIZE),
    });
}
pub fn spawn_masks(mut commands: Commands,
                    time: Res<Time>,
                    mut spawner: ResMut<MaskSpawner>,
                    asset_server: Res<AssetServer>,
                    masks: Query<Entity, With<Mask>>,) {
    if masks.iter().len() >= spawner.max_masks {
        return;
    }
    spawner.timer.tick(time.delta());
    if !spawner.timer.just_finished() {return;}

    let mut rng = rand::rng();
    let x = rng.random_range(spawner.min.x..spawner.max.x);
    let y = rng.random_range(spawner.min.y..spawner.max.y);

    commands.spawn((
        Mask,
        Sprite {
            custom_size: Some(Vec2::splat(75.)),
            image: asset_server.load("img/mask.png"),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Hitbox{
            size: Vec2::new(32.0, 64.0),
            offset: Vec2::ZERO,
        }
    )).insert(LevelEntity);
}
