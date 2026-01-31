use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

