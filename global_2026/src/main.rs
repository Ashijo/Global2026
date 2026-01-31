mod map;
mod player;
mod enemy;
mod item;

use bevy::prelude::*;
use bevy::camera::ScalingMode;
use bevy::prelude::OrthographicProjection;

const WINDOW_WIDTH:f32 = 1920.0;
const WINDOW_HEIGHT:f32 = 1080.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (main_setup, map::map_setup,enemy::enemy_setup,item::item_setup, player::player_setup))
        .add_systems(Update, (map::map_update,enemy::enemy_update,item::item_update, player::player_update, player::player_animation))
        .add_systems(FixedUpdate, (map::map_fixed_update,enemy::enemy_fixed_update,item::item_fixed_update, player::player_fixed_update))
        .run();
}

fn main_setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn((Camera2d,
                   Projection::Orthographic(OrthographicProjection {
                       viewport_origin: Vec2::new(0.0, 0.0),
                       scaling_mode:ScalingMode::AutoMax {
                           max_width: WINDOW_WIDTH,
                           max_height: WINDOW_HEIGHT
                       },
                       ..OrthographicProjection::default_2d()
                   }) ));
}