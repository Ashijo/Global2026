use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::collision::Hitbox;
use crate::GameState;
use crate::level::LevelEntity;
use crate::stunned::Stunned;
use crate::player::{Player, HasMask};
use crate::unmasked::UnmaskedScore;

const ENEMY_VELOCITY: f32 = 320.0;
const EPSILON: f32 = 5.0;
const TARGET_FUSE_TIME:[u64; 4] = [1,3,5,6];

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_setup);
        app.add_systems(
            FixedUpdate,
            (
                random_target_spawner,
                enemy_animation,
                enemy_movement,
                collide_player,
                detect_player,
            )
                .chain(),
        );

    }
}

pub fn enemy_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("img/cop_2.png");

    // from_grid define spritesheet division ( tile_size: UVec2,
    //     columns: u32,
    //     rows: u32,
    //     padding: Option<UVec2>,
    //     offset: Option<UVec2>)
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {  
            x: 49,
            y: 65
        },
        9,
        1,
        None,
        None
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices_1 = AnimationIndices { first: 0, last: 8 };
    let animation_indices_2 = AnimationIndices { first: 0, last: 8 };
    let animation_indices_3 = AnimationIndices { first: 0, last: 8 };

    let mut transform_1 = Transform::from_scale(Vec3::splat(1.0));
    let mut transform_2 = Transform::from_scale(Vec3::splat(1.0));
    let mut transform_3 = Transform::from_scale(Vec3::splat(1.0));

    transform_1.translation = Vec3::new(1750.0, 100.0, 1.0);
    transform_2.translation = Vec3::new(1750.0, 500.0, 1.0);
    transform_3.translation = Vec3::new(1750.0, 800.0, 1.0);

    commands
        .spawn((
            Enemy,
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_indices_1.first,
                },
            ),
            transform_1,
            animation_indices_1,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
            FuseTime {
                timer: Timer::new(Duration::from_secs(3), TimerMode::Once)
            }
        ))
        .insert(LevelEntity);

    commands
        .spawn((
            Enemy,
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_indices_2.first,
                },
            ),
            transform_2,
            animation_indices_2,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
            FuseTime {
                timer: Timer::new(Duration::from_secs(2), TimerMode::Once)
            }
        ))
        .insert(LevelEntity);

    commands
        .spawn((
            Enemy,
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_indices_3.first,
                },
            ),
            transform_3,
            animation_indices_3,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
            FuseTime {
                timer: Timer::new(Duration::from_secs(4), TimerMode::Once)
            }
        ))
        .insert(LevelEntity);
}

pub fn enemy_animation(
    time: Res<Time>,
    mut animation: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut animation {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &Target, Option<&Stunned>), (With<Target>, With<Enemy>)>,
) {
    for (entity, mut transform, target, stunned) in &mut enemy_query {
        if stunned.is_some() {
            continue;
        }

        if !close_to_target(target, *transform, EPSILON) {
            let mut dir = Vec2::ZERO;

            if !eps_x(target, *transform, EPSILON) {
                if target.pos.x < transform.translation.x {
                    dir.x -= 1.0;
                } else if target.pos.x > transform.translation.x {
                    dir.x += 1.0;
                }
            }

            if !eps_y(target, *transform, EPSILON) {
                if target.pos.y < transform.translation.y {
                    dir.y -= 1.0;
                } else if target.pos.y > transform.translation.y {
                    dir.y += 1.0;
                }
            }

            if dir != Vec2::ZERO {
                dir = dir.normalize();
                let dt = time.delta_secs();

                transform.translation.x += dir.x * ENEMY_VELOCITY * dt;
                transform.translation.y += dir.y * ENEMY_VELOCITY * dt;
            }
        } else {
            commands.entity(entity).remove::<Target>();
            
            let mut rng = rand::rng();
            let fuse_time = TARGET_FUSE_TIME.choose(&mut rng).unwrap();

            commands.entity(entity).insert(FuseTime{
                timer: Timer::new(Duration::from_secs(*fuse_time), TimerMode::Once),
            });
        }
    }
}

fn collide_player(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Hitbox, Option<&Stunned>), With<Enemy>>,
    player_transform: Single<&Transform, With<Player>>,
    player_hitbox: Single<&Hitbox, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut unmasked_score: ResMut<UnmaskedScore>,
) {
    let mut kill = false;

    let p_min_x =
        player_transform.translation.x - player_hitbox.size.x / 2.0 + player_hitbox.offset.x;
    let p_max_x =
        player_transform.translation.x + player_hitbox.size.x / 2.0 + player_hitbox.offset.x;
    let p_min_y =
        player_transform.translation.y - player_hitbox.size.y / 2.0 + player_hitbox.offset.y;
    let p_max_y =
        player_transform.translation.y + player_hitbox.size.y / 2.0 + player_hitbox.offset.y;

    for (enemy_entity, en_trans, hitbox, stunned) in &enemy_query {
        let en_min_x = en_trans.translation.x - hitbox.size.x / 2.0;
        let en_max_x = en_trans.translation.x + hitbox.size.x / 2.0;
        let en_min_y = en_trans.translation.y - hitbox.size.y / 2.0;
        let en_max_y = en_trans.translation.y + hitbox.size.y / 2.0;

        let x_overlap = p_min_x < en_max_x && p_max_x > en_min_x;
        let y_overlap = p_min_y < en_max_y && p_max_y > en_min_y;

        if x_overlap && y_overlap {
            if stunned.is_some() {
                commands.entity(enemy_entity).despawn();
                unmasked_score.0 += 1; // increment score
                
                if unmasked_score.0 == 3 {
                    next_state.set(GameState::GameOver);
                }

                println!("💀 Enemy killed while stunned!");
            } else {
                kill = true;
            }
        }
    }

    if kill {
        if cfg!(debug_assertions) {
            println!("DEAD !!!");
        }
        else {
            next_state.set(GameState::GameOver);
            //println!("{:?}", next_state);
        }
    }
}

fn detect_player(
    mut commands: Commands,
    player_transform: Single<&Transform, With<Player>>,
    player_has_mask: Single<&HasMask, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &Detection, Option<&Target>), With<Enemy>>,
    chasing_query: Query<Entity, (With<Enemy>, With<Target>)>,
) {
    if player_has_mask.0 {
        let mut rng = rand::rng();

        for e in &chasing_query {
            commands.entity(e).remove::<Target>();

            let fuse_time = TARGET_FUSE_TIME.choose(&mut rng).unwrap();

            commands.entity(e).insert(FuseTime{
                timer: Timer::new(Duration::from_secs(*fuse_time), TimerMode::Once),
            });
        }
        return;
    }
    for (entity, en_trans, detection, target) in &mut enemy_query {
        let en_min_x = en_trans.translation.x - detection.size.x / 2.0;
        let en_max_x = en_trans.translation.x + detection.size.x / 2.0;
        let en_min_y = en_trans.translation.y - detection.size.y / 2.0;
        let en_max_y = en_trans.translation.y + detection.size.y / 2.0;

        let x_overlap =
            player_transform.translation.x < en_max_x && player_transform.translation.x > en_min_x;
        let y_overlap =
            player_transform.translation.y < en_max_y && player_transform.translation.y > en_min_y;

        let detect = x_overlap && y_overlap;

        if detect {
            if target.is_some() {
                commands.entity(entity).remove::<Target>();
            }

            commands.entity(entity).insert(Target{
                pos: Vec2::new(player_transform.translation.x, player_transform.translation.y),
            });
        }
    }
}

fn random_target_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut FuseTime), (Without<Target>, With<Enemy>)>
) {
    for (entity, mut fuse_timer) in enemy_query.iter_mut() {
        fuse_timer.timer.tick(time.delta());

        if fuse_timer.timer.just_finished() {
            let mut rng = rand::rng();
            let target = Target{
                pos: Vec2{
                    x: rng.random_range(100.0..1800.0),
                    y: rng.random_range(100.0..800.0)
                }
            };

            commands.entity(entity).insert(target);
            commands.entity(entity).remove::<FuseTime>();
        }
    }
}

fn close_to_target(target: &Target, trans: Transform, eps: f32) -> bool {
    eps_x(target, trans, eps) && eps_y(target, trans, eps)
}

fn eps_x(target: &Target, trans: Transform, eps: f32) -> bool {
    return target.pos.x + eps >= trans.translation.x && target.pos.x - eps <= trans.translation.x;
}

fn eps_y(target: &Target, trans: Transform, eps: f32) -> bool {
    target.pos.y + eps >= trans.translation.y && target.pos.y - eps <= trans.translation.y
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Detection {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Target {
    pos: Vec2,
}


#[derive(Component)]
struct FuseTime {
    /// non-repeating timer
    timer: Timer,
}


#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
