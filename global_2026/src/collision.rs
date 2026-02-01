use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2,
}