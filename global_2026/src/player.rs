use bevy::prelude::*;
use crate::collision::Hitbox;
use crate::mask::Mask;
use crate::stunned::Stunned;
use crate::level::LevelEntity;

const SPEED: f32 = 300.0;

// Taille d'UNE frame de ton spritesheet
const TILE_WIDTH: u32 = 278/4;
const TILE_HEIGHT: u32 = 384/3;

// Spritesheet : 4 colonnes (frames), 3 lignes (Down/Side/Up)
const COLS: usize = 4;
const ROWS: usize = 3;

// Vitesse de l'animation (frames/sec)
const ANIM_FPS: f32 = 10.0;


const WINDOW_WIDTH:f32 = 1920.0;
const WINDOW_HEIGHT:f32 = 1080.0;

const MAX_BOTTOM: f32 = 100.0;
const MAX_TOP: f32 = 100.0;
const MAX_LEFT: f32 = 100.0;
const MAX_RIGHT: f32 = 100.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Anim {
    first: usize,
    last: usize,
    timer: Timer,
    playing: bool,
}

#[derive(Component, Debug)]
pub struct HasMask(pub bool);

#[derive(Component)]
pub struct MaskTimer {
    pub timer: Timer,
}

#[derive(Component, Clone, Copy)]
pub enum Facing {
    Down,
    Side,
    Up,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(
            FixedUpdate,
            (
                player_fixed_update,
                player_animation,
                pickup_mask,
                mask_timer_update,
            )
                .chain(),
        );
    }
}

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("img/bomberman-sprite-sheet.png");

    // Découpage en grille (frame_w, frame_h, cols, rows)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH, TILE_HEIGHT),
        COLS as u32,
        ROWS as u32,
        None,
        None,
    );
    let layout_handle = layouts.add(layout);

    let start_index = 0; // première frame de Down

    commands.spawn((
        Player,
        HasMask(false),
        Facing::Down,
        Anim {
            first: 0,
            last: COLS - 1, // frames 0..3 pour la ligne 0
            timer: Timer::from_seconds(1.0 / ANIM_FPS, TimerMode::Repeating),
            playing: false,
        },
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: layout_handle,
                index: start_index,
            },
        ),
        Transform::from_xyz(100.0, 490.0, 1.0),
        Hitbox {
            size: Vec2::new(32.0, 64.0),
            offset: Vec2::new(0.0, -12.0),
        },
    )).insert(Player).insert(LevelEntity);
}

pub fn player_fixed_update(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut Facing, &mut Anim, &mut Sprite, Option<&Stunned>), With<Player>>,
) {
    let Ok((mut tf, mut facing, mut anim, mut sprite, stunned)) = q.single_mut() else {
        return;
    };

    if stunned.is_some() {
        anim.playing = false;
        return;
    }

    let mut dir = Vec2::ZERO;

    let max_left_reached = tf.translation.x <= MAX_LEFT;
    let max_right_reached = tf.translation.x >= WINDOW_WIDTH - MAX_RIGHT;
    let max_top_reached = tf.translation.y >= WINDOW_HEIGHT - MAX_TOP;
    let max_bottom_reached = tf.translation.y <= MAX_BOTTOM;


    if keys.pressed(KeyCode::KeyA) && !max_left_reached { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) && !max_right_reached { dir.x += 1.0; }
    if keys.pressed(KeyCode::KeyS) && !max_bottom_reached { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyW) && !max_top_reached { dir.y += 1.0; }

    if dir != Vec2::ZERO {
        dir = dir.normalize();
        anim.playing = true;

        let vertical = keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::KeyS);
        let horizontal = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::KeyD);

        // On check dans quelle direction l'utilisateur veut aller
        if vertical && !horizontal {
            sprite.flip_x = false;
            *facing = if dir.y > 0.0 { Facing::Up } else { Facing::Down };
        } else if horizontal && !vertical {
            *facing = Facing::Side;
        // flipper car notre sprite n'a que le déplacement à droite
            sprite.flip_x = dir.x < 0.0;
        } else if vertical && horizontal {
        // diagonale -> priorité verticale
            sprite.flip_x = false;
            *facing = if dir.y > 0.0 { Facing::Up } else { Facing::Down };
        }

        // mémoriser l’ancienne ligne du sprite sheet pour détecter un changement de direction
        let old_first = anim.first;

        let row = match *facing {
            Facing::Up => 0,
            Facing::Side => 1,
            Facing::Down => 2,
        };

        anim.first = row * COLS;
        anim.last = anim.first + (COLS - 1);

        //raflaîchir pour changer immédiatement de direction si on change de direction
        if anim.first != old_first {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = anim.first;
            }
            anim.timer.reset();
        }
    } else {
        anim.playing = false;
    }


    let dt = time.delta_secs();
    tf.translation.x += dir.x * SPEED * dt;
    tf.translation.y += dir.y * SPEED * dt;
}

pub fn player_animation(
    time: Res<Time>,
    mut q: Query<(&mut Sprite, &mut Anim), With<Player>>,
) {
    let Ok((mut sprite, mut anim)) = q.single_mut() else {
        return;
    };

    let Some(atlas) = sprite.texture_atlas.as_mut() else {
        return;
    };

    if !anim.playing {
        atlas.index = anim.first;
        return;
    }

    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
        atlas.index = if atlas.index >= anim.last { anim.first } else { atlas.index + 1 };
    }
}

pub fn pickup_mask(
    mut commands: Commands,
    mut player_q: Query<(Entity, &Transform, &Hitbox, &mut HasMask), With<Player>>,
    mask_q: Query<(Entity, &Transform, &Hitbox), With<Mask>>,
) {
    let Ok((player_entity, p_tf, p_hb, mut has_mask)) = player_q.single_mut() else { return; };
    if has_mask.0 {
        return;
    }

    let p_center = p_tf.translation.truncate() + p_hb.offset;
    let p_half = p_hb.size * 0.5;

    for (mask_e, m_tf, m_hb) in &mask_q {
        let m_center = m_tf.translation.truncate() + m_hb.offset;
        let m_half = m_hb.size * 0.5;

        let delta = (p_center - m_center).abs();

        let overlap =
            delta.x < (p_half.x + m_half.x) &&
                delta.y < (p_half.y + m_half.y);

        if overlap {
            has_mask.0 = true;
            commands.entity(mask_e).despawn();
            commands.entity(player_entity).insert(
                MaskTimer {
                    timer: Timer::from_seconds(8.0, TimerMode::Once),
                }
            );
            println!("You picked up a mask");
            break;
        }
    }
}

pub fn mask_timer_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut HasMask, &mut MaskTimer), With<Player>>,
) {
    let Some((entity, mut has_mask, mut mask_timer)) = q.iter_mut().next() else {
        return; // aucun joueur (ou pas de MaskTimer) => rien à faire
    };

    mask_timer.timer.tick(time.delta());
    if mask_timer.timer.just_finished() {
        has_mask.0 = false;
        println!("Time's up! Your mask is kaput...");
        commands.entity(entity).remove::<MaskTimer>();
    }
}
