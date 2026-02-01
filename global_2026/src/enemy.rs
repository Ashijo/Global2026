use bevy::{color::palettes::css::*, prelude::*} ;
use rand::prelude::*;

use crate::collision::Hitbox;
use crate::level::LevelEntity;
use crate::player::Player;

const ENEMY_VELOCITY: f32 = 320.0;
const EPSILON: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_setup);
        app.add_systems(FixedUpdate, (enemy_fixed_update, collide_player, detect_player, gizmo_hitbox, gizmo_detection).chain());
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

    commands.spawn((
        Enemy {
            target: None
        },
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
        },Detection {
            size: Vec2::splat(336.0)
        },
    )).insert(LevelEntity);

    commands.spawn((
        Enemy {
            target: None
        },
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
        },Detection {
            size: Vec2::splat(336.0)
        },
    )).insert(LevelEntity);

    commands.spawn((
        Enemy {
            target: None
        },
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
            size: Vec2::splat(336.0)
        },
    )).insert(LevelEntity);
}

pub fn enemy_fixed_update(
    time: Res<Time>,
    mut animation: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform)>,
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


    for (mut enemy, mut transform) in &mut enemy_query {
        match &enemy.target {
            Some(value) => {


                if !close_to_target(value, *transform, EPSILON) {
                    let mut dir = Vec2::ZERO;

                    if !eps_x(value, *transform, EPSILON) {
                        if value.x < transform.translation.x {
                            dir.x -= 1.0;
                        } else if value.x > transform.translation.x {
                            dir.x += 1.0;
                        }
                    }

                    if !eps_y(value, *transform, EPSILON) {
                        if value.y < transform.translation.y {
                            dir.y -= 1.0;
                        } else if value.y > transform.translation.y {
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
                    enemy.target = None;
                }
            }
            None => {
                // Generate and shuffle a sequence:
                let nums_h: Vec<u32> = (1..1920).collect();
                let nums_v: Vec<u32> = (1..1080).collect();
                let mut rng = rand::rng();
                let new_x = nums_h.choose(&mut rng);
                let new_y = nums_v.choose(&mut rng);

                enemy.target = Some(Target {
                    x: *new_x.unwrap() as f32,
                    y: *new_y.unwrap() as f32,
                });
            }
        }
    }
}

fn gizmo_hitbox(
    mut gizmos: Gizmos,
    hitbox_query: Query<(&Hitbox, &Transform)>
) {
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


fn gizmo_detection(
    mut gizmos: Gizmos,
    detection_query: Query<(&Detection, &Transform)>
) {
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

    let p_min_x = player_transform.translation.x - player_hitbox.size.x / 2.0 + player_hitbox.offset.x;
    let p_max_x = player_transform.translation.x + player_hitbox.size.x / 2.0  + player_hitbox.offset.x;
    let p_min_y = player_transform.translation.y - player_hitbox.size.y / 2.0 + player_hitbox.offset.y;
    let p_max_y = player_transform.translation.y + player_hitbox.size.y / 2.0 + player_hitbox.offset.y;

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
    player_transform: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &Detection, &mut Enemy)>
) {
    for (en_trans, detection, mut enemy) in &mut enemy_query {
        let en_min_x = en_trans.translation.x - detection.size.x / 2.0;
        let en_max_x = en_trans.translation.x + detection.size.x / 2.0;
        let en_min_y = en_trans.translation.y - detection.size.y / 2.0;
        let en_max_y = en_trans.translation.y + detection.size.y / 2.0;

        let x_overlap = player_transform.translation.x < en_max_x && player_transform.translation.x > en_min_x;
        let y_overlap = player_transform.translation.y < en_max_y && player_transform.translation.y > en_min_y;

        let detect = x_overlap && y_overlap;

        if detect {
            enemy.target = Some(Target{x: player_transform.translation.x, y:player_transform.translation.y});
        }
    }
}

fn close_to_target(target: &Target, trans: Transform, eps: f32) -> bool {
    eps_x(target, trans, eps) && eps_y(target, trans, eps)
}

fn eps_x(target: &Target, trans: Transform, eps: f32) -> bool {
    target.x + eps >= trans.translation.x && target.x - eps <= trans.translation.x
}

fn eps_y(target: &Target, trans: Transform, eps: f32) -> bool {
    target.y + eps >= trans.translation.y && target.y - eps <= trans.translation.y
}

#[derive(Component)]
pub struct Enemy {
    target: Option<Target>
}

#[derive(Component)]
struct Detection {
    size: Vec2
}

pub struct Target {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
