use bevy::{color::palettes::css::*, prelude::*};
// use rand::prelude::*;

use crate::collision::Hitbox;
use crate::level::LevelEntity;
use crate::player::{Player, HasMask};

const ENEMY_VELOCITY: f32 = 320.0;
const EPSILON: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_setup);
        app.add_systems(
            FixedUpdate,
            (
                enemy_animation,
                enemy_movement,
                collide_player,
                detect_player,
                gizmo_hitbox,
                gizmo_detection,
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
    let texture = asset_server.load("img/enemy.png");

    // from_grid define spritesheet division ( tile_size: UVec2,
    //     columns: u32,
    //     rows: u32,
    //     padding: Option<UVec2>,
    //     offset: Option<UVec2>)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        13,
        1,
        Some(UVec2::splat(1)),
        Some(UVec2::splat(1)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices_1 = AnimationIndices { first: 0, last: 2 };
    let animation_indices_2 = AnimationIndices { first: 3, last: 5 };
    let animation_indices_3 = AnimationIndices { first: 6, last: 8 };

    let mut transform_1 = Transform::from_scale(Vec3::splat(4.0));
    let mut transform_2 = Transform::from_scale(Vec3::splat(4.0));
    let mut transform_3 = Transform::from_scale(Vec3::splat(4.0));

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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Hitbox {
                size: Vec2::splat(64.0),
                offset: Vec2::ZERO,
            },
            Detection {
                size: Vec2::splat(336.0),
            },
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
    mut enemy_query: Query<(Entity, &mut Transform, &Target), (With<Target>, With<Enemy>)>,
) {
    for (entity, mut transform, target) in &mut enemy_query {
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
        }
    }
}

fn gizmo_hitbox(mut gizmos: Gizmos, hitbox_query: Query<(&Hitbox, &Transform)>) {
    for (hitbox, transform) in &hitbox_query {
        let min_x = transform.translation.x - hitbox.size.x / 2.0 + hitbox.offset.x;
        let max_x = transform.translation.x + hitbox.size.x / 2.0 + hitbox.offset.x;
        let min_y = transform.translation.y - hitbox.size.y / 2.0 + hitbox.offset.y;
        let max_y = transform.translation.y + hitbox.size.y / 2.0 + hitbox.offset.y;

        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(min_x, max_y), RED);
        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(max_x, min_y), RED);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(min_x, max_y), RED);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(max_x, min_y), RED);
    }
}

fn gizmo_detection(mut gizmos: Gizmos, detection_query: Query<(&Detection, &Transform)>) {
    for (detection, transform) in &detection_query {
        let min_x = transform.translation.x - detection.size.x / 2.0;
        let max_x = transform.translation.x + detection.size.x / 2.0;
        let min_y = transform.translation.y - detection.size.y / 2.0;
        let max_y = transform.translation.y + detection.size.y / 2.0;

        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(min_x, max_y), GREEN);
        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(max_x, min_y), GREEN);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(min_x, max_y), GREEN);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(max_x, min_y), GREEN);
    }
}

fn collide_player(
    enemy_query: Query<(&Transform, &Hitbox), With<Enemy>>,
    player_transform: Single<&Transform, With<Player>>,
    player_hitbox: Single<&Hitbox, With<Player>>,
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

    for (en_trans, hitbox) in &enemy_query {
        let en_min_x = en_trans.translation.x - hitbox.size.x / 2.0;
        let en_max_x = en_trans.translation.x + hitbox.size.x / 2.0;
        let en_min_y = en_trans.translation.y - hitbox.size.y / 2.0;
        let en_max_y = en_trans.translation.y + hitbox.size.y / 2.0;

        let x_overlap = p_min_x < en_max_x && p_max_x > en_min_x;
        let y_overlap = p_min_y < en_max_y && p_max_y > en_min_y;

        kill = x_overlap && y_overlap;
    }

    if kill {
        println!("DEAD !!!");
    }
}

fn detect_player(
    mut commands: Commands,
    player_transform: Single<&Transform, With<Player>>,
    player_has_mask: Single<&HasMask, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &Detection), (Without<Target>, With<Enemy>)>,
    mut chasing_query: Query<Entity, (With<Enemy>, With<Target>)>,
) {
    if player_has_mask.0 {
        for e in &chasing_query {
             commands.entity(e).remove::<Target>();
        }
        return;
    }
    for (entity, en_trans, detection) in &mut enemy_query {
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
            commands.entity(entity).insert(Target{
                pos: Vec2::new(player_transform.translation.x, player_transform.translation.y),
            });

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
struct Detection {
    size: Vec2,
}

#[derive(Component)]
pub struct Target {
    pos: Vec2,
}

/*
#[derive(Component)]
struct FuseTime {
    /// non-repeating timer
    timer: Timer,
}
*/

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
