use bevy::prelude::*;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::bomb::{BOMB_SIZE, BOMB_Z};
use crate::collision::Hitbox;
use crate::stunned::{Stunned, STUN_DURATION};
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
        Hitbox { size: Vec2::splat(BLAST_SIZE) },
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
            Hitbox { size: Vec2::splat(BLAST_SIZE) },
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
            Hitbox { size: Vec2::splat(BLAST_SIZE) },
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
            Hitbox { size: Vec2::splat(BLAST_SIZE) },
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
            Hitbox { size: Vec2::splat(BLAST_SIZE) },
        ));
    }
}

pub fn blast_collision_system(
    mut commands: Commands,
    blast_query: Query<(&Transform, &Hitbox), With<Blast>>,
    target_query: Query<
        (
            Entity,
            &Transform,
            &Hitbox,
            Option<&Player>,
            Option<&Enemy>,
            Option<&Stunned>,
        ),
    >,
) {
    for (blast_tf, blast_hitbox) in blast_query.iter() {
        let blast_pos = blast_tf.translation.truncate();
        let blast_half = blast_hitbox.size * 0.5;

        for (entity, target_tf, target_hitbox, player, enemy, stunned) in target_query.iter() {
            // Ignore entities that are neither Player nor Enemy
            if player.is_none() && enemy.is_none() {
                continue;
            }

            let target_pos = target_tf.translation.truncate();
            let target_half = target_hitbox.size * 0.5;

            let delta = blast_pos - target_pos;

            let overlap_x = delta.x.abs() < (blast_half.x + target_half.x);
            let overlap_y = delta.y.abs() < (blast_half.y + target_half.y);

            if overlap_x && overlap_y {
                if stunned.is_none() {
                    commands.entity(entity).insert(Stunned {
                        timer: Timer::from_seconds(STUN_DURATION, TimerMode::Once),
                    });
                    if player.is_some() {
                        println!("🔥 Player hit by blast");
                    } else if enemy.is_some() {
                        println!("🔥 Enemy {:?} hit by blast", entity);
                    }
                }
            }
        }
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
