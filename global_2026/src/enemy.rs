use bevy::prelude::*;
use rand::prelude::*;

const ENEMY_VELOCITY: f32 = 400.0;
const EPSILON: f32 = 15.0;

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
    let mut transform = Transform::from_scale(Vec3::splat(4.0));
    transform.translation = Vec3::new(150.0, 70.0, 1.0);

    commands.spawn((
        Enemy { target: None, id: 1 },
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_indices_1.first,
            },
        ),
        transform,
        animation_indices_1,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    ));

    commands.spawn((
        Enemy { target: None, id: 2 },
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_indices_2.first,
            },
        ),
        transform,
        animation_indices_2,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    ));

    commands.spawn((
        Enemy { target: None, id: 3 },
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_indices_3.first,
            },
        ),
        transform,
        animation_indices_3,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    ));
}

pub fn enemy_update() {}

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
    target: Option<Target>,
    id:u32
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
