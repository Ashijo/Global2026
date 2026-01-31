use bevy::prelude::*;
use crate::bomb::{Bomb, BOMB_SIZE, BOMB_Z};
pub const BLAST_SIZE: f32 = BOMB_SIZE;
pub const BLAST_LENGTH: u32 = 3;
pub const BLAST_DURATION: f32 = 1.0;
pub const BLAST_Z: f32 = BOMB_Z + 0.01;

#[derive(Component)]
pub struct Blast;

#[derive(Component)]
pub struct BlastTimer {
    pub timer: Timer,
}

// spawn blast centered around bomb
pub fn spawn_blast(commands: &mut Commands, asset_server: &Res<AssetServer>, center: Vec3) {
    let blast_texture = asset_server.load("img/bomb.png");

    //spawn center blast
    commands.spawn((
        Blast,
        BlastTimer {
            timer: Timer::from_seconds(BLAST_DURATION, TimerMode::Once),
        },
        Sprite {
            image: blast_texture.clone(),
            custom_size: Some(Vec2::splat(BLAST_SIZE)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, BLAST_Z),
    ));

    // spawn blast arms
    for i in 1..=BLAST_LENGTH {
        let offset = i as f32 * BLAST_SIZE;

        // Up
        commands.spawn((
            Blast,
            BlastTimer {
                timer: Timer::from_seconds(BLAST_DURATION, TimerMode::Once),
            },
            Sprite {
                image: blast_texture.clone(),
                custom_size: Some(Vec2::splat(BLAST_SIZE)),
                ..default()
            },
            Transform::from_xyz(center.x, center.y + offset, BLAST_Z),
        ));

        // Down
        commands.spawn((
            Blast,
            BlastTimer {
                timer: Timer::from_seconds(BLAST_DURATION, TimerMode::Once),
            },
            Sprite {
                image: blast_texture.clone(),
                custom_size: Some(Vec2::splat(BLAST_SIZE)),
                ..default()
            },
            Transform::from_xyz(center.x, center.y - offset, BLAST_Z),
        ));

        // Right
        commands.spawn((
            Blast,
            BlastTimer {
                timer: Timer::from_seconds(BLAST_DURATION, TimerMode::Once),
            },
            Sprite {
                image: blast_texture.clone(),
                custom_size: Some(Vec2::splat(BLAST_SIZE)),
                ..default()
            },
            Transform::from_xyz(center.x + offset, center.y, BLAST_Z),
        ));

        // Left
        commands.spawn((
            Blast,
            BlastTimer {
                timer: Timer::from_seconds(BLAST_DURATION, TimerMode::Once),
            },
            Sprite {
                image: blast_texture.clone(),
                custom_size: Some(Vec2::splat(BLAST_SIZE)),
                ..default()
            },
            Transform::from_xyz(center.x - offset, center.y, BLAST_Z),
        ));
    }
}

pub fn blast_update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BlastTimer), With<Blast>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
