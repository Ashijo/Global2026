mod map;
mod player;
mod enemy;
mod item;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (map::map_setup,enemy::enemy_setup,item::item_setup, player::player_setup))
        .add_systems(Update, (map::map_update,enemy::enemy_update,item::item_update, player::player_update))
        .add_systems(FixedUpdate, (map::map_fixed_update,enemy::enemy_fixed_update,item::item_fixed_update, player::player_fixed_update))
        .run();
}

