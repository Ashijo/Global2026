mod map;
mod player;
mod enemy;
mod bomb;
mod hud;
mod mask;
mod blast;
mod collision;
mod level;

use bevy::prelude::*;
use bevy::camera::ScalingMode;
use bevy::prelude::OrthographicProjection;
use crate::enemy::EnemyPlugin;

use crate::level::LevelEntity;
use crate::level::despawn_entities;

const WINDOW_WIDTH:f32 = 1920.0;
const WINDOW_HEIGHT:f32 = 1080.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnemyPlugin)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::GameOver), restart_game)
        .add_systems(Startup, (main_setup, map::map_setup,bomb::bomb_setup, player::player_setup, hud::hud_setup, mask::mask_setup))
        .add_systems(Update, (map::map_update,bomb::bomb_update, player::player_update, blast::blast_update, hud::hud_update))
        .add_systems(FixedUpdate, (map::map_fixed_update,bomb::bomb_fixed_update, player::player_fixed_update, player::player_animation, mask::spawn_masks, blast::blast_collision_system,).chain())
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

fn restart_game(mut commands: Commands, entities: Query<Entity, With<LevelEntity>>)
{
    despawn_entities(&mut commands, entities.iter());
}