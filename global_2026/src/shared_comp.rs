use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}


pub fn hello() {
    println!("Hello, world!");
}
