use bevy::prelude::*;
use crate::player::Player;
use crate::blast::spawn_blast;

const BOMB_LIFETIME_SECS: f32 = 3.0;
pub const BOMB_SIZE: f32 = 55.0;
pub const BOMB_Z: f32 = 0.5;
const FLASH_INTERVAL_SLOW: f32 = 0.6; // start slow
const FLASH_INTERVAL_FAST: f32 = 0.05; // end fast

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct BombTimer {
    pub timer: Timer,
}
#[derive(Component)]
pub struct BombFlash {
    pub flash_timer: Timer,
    pub normal_texture: Handle<Image>,
    pub white_texture: Handle<Image>,
}

pub fn bomb_setup() {
//    println!("item setup")
}

pub fn bomb_update(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,

    player_query: Query<&Transform, With<Player>>,

    bombs: Query<Entity, With<Bomb>>,
    mut bomb_query: Query<(Entity, &mut BombTimer, &mut BombFlash, &mut Sprite, &Transform)>,
) {
    // Spawn bomb under player if none exists
    if keys.just_pressed(KeyCode::Space) && bombs.is_empty() {
        let player_transform = match player_query.iter().next() {
            Some(transform) => transform,
            None => return,
        };

        let normal_texture = asset_server.load("img/bomb_2.png");
        let white_texture = asset_server.load("img/bomb_white_2.png");

        commands.spawn((
            Bomb,
            BombTimer {
                timer: Timer::from_seconds(BOMB_LIFETIME_SECS, TimerMode::Once),
            },
            BombFlash {
                flash_timer: Timer::from_seconds(FLASH_INTERVAL_SLOW, TimerMode::Repeating),
                normal_texture: normal_texture.clone(),
                white_texture: white_texture.clone(),
            },
            Sprite {
                image: normal_texture.clone(),
                custom_size: Some(Vec2::splat(BOMB_SIZE)),
                ..default()
            },
            Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y,
                BOMB_Z,
            ),
        ));
    }

    //Update bombs timer and flashing
    for (entity, mut bomb_timer, mut flash, mut sprite, &transform) in bomb_query.iter_mut() {
        // Advance bomb lifetime timer
        bomb_timer.timer.tick(time.delta());

        // Despawn bomb when timer finishes
        if bomb_timer.timer.just_finished() {
            spawn_blast(&mut commands, &asset_server, transform.translation);
            commands.entity(entity).despawn();
            continue;
        }

        let remaining = bomb_timer.timer.remaining_secs();
        let progress = (remaining / BOMB_LIFETIME_SECS).clamp(0.0, 1.0);
        let flash_interval = FLASH_INTERVAL_FAST + (FLASH_INTERVAL_SLOW - FLASH_INTERVAL_FAST) * progress;
        flash.flash_timer.set_duration(std::time::Duration::from_secs_f32(flash_interval));

        flash.flash_timer.tick(time.delta());

        if flash.flash_timer.just_finished() {
            sprite.image = if sprite.image == flash.normal_texture {
                flash.white_texture.clone()
            } else {
                flash.normal_texture.clone()
            };
        }
    }
}

pub fn bomb_fixed_update() {
//    println!("item fixed update")
}

