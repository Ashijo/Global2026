mod map;
mod player;
mod enemy;
mod gizmos;
mod bomb;
mod hud;
mod gameover;
mod mask;
mod blast;
mod collision;
mod level;
mod stunned;
mod unmasked;

use bevy::prelude::*;
use bevy::camera::ScalingMode;
use bevy::prelude::OrthographicProjection;
use bevy::app::AppExit;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::gizmos::GizmosPlugin;
use crate::unmasked::UnmaskedScore;

use crate::level::LevelEntity;
use crate::level::despawn_entities;

const WINDOW_WIDTH:f32 = 1920.0;
const WINDOW_HEIGHT:f32 = 1080.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Playing,
    Restart,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GizmosPlugin)
        .init_state::<GameState>()
        .insert_resource(UnmaskedScore::default())
        .add_systems(OnEnter(GameState::GameOver), gameover::gameover_setup)
        .add_systems(OnEnter(GameState::Restart), (restart_game, player::player_setup, enemy::enemy_setup))
        .add_systems(Startup, (main_setup, map::map_setup,bomb::bomb_setup, hud::hud_setup, mask::mask_setup))
        .add_systems(Update, (bomb::bomb_update, blast::blast_update, hud::hud_update, hud::hud_score_update))
        .add_systems(FixedUpdate, (bomb::bomb_fixed_update,
                                   mask::spawn_masks,
                                   blast::blast_collision_system, 
                                   stunned::stun_update,
        ).chain())
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

fn end_game(
) {

    println!("GAEME OVER");
}
