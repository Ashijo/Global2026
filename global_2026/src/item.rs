use bevy::prelude::*;

const BOMB_LIFETIME_SECS: f32 = 5.0;
const BOMB_SIZE: f32 = 25.0;
const BOMB_Z: f32 = 1.0;

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct BombTimer {
    pub timer: Timer,
}

pub fn item_setup() {
    println!("item setup")
}

pub fn item_update(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,

    bombs: Query<Entity, With<Bomb>>,
    mut bomb_timers: Query<(Entity, &mut BombTimer)>,
) {
    if keys.just_pressed(KeyCode::Space) && bombs.is_empty() {
        commands.spawn((
            Bomb,
            BombTimer {
                timer: Timer::from_seconds(BOMB_LIFETIME_SECS, TimerMode::Once),
            },
            Sprite {
                image: asset_server.load("test.png"),
                custom_size: Some(Vec2::splat(BOMB_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, BOMB_Z),
        ));
    }

    for (entity, mut bomb_timer) in bomb_timers.iter_mut() {
        bomb_timer.timer.tick(time.delta());

        if bomb_timer.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn item_fixed_update() {
    println!("item fixed update")
}

