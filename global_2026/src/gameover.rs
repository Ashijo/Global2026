use bevy::prelude::*;
use crate::GameState;

pub fn gameover_setup(mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    /*
    commands.spawn((
        GameOverComp,
        Text::new("Game Over"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(500.0),
            right: Val::Px(500.0),
            ..default()
        },
    ));
*/

    next_state.set(GameState::Restart);

}


pub fn reset_gameover(
    mut commands: Commands,
    mut player_entity: Single<Entity, With<GameOverComp>>) {
   
    let (e) = player_entity;
    println!("should despawn");
    commands.entity(*e).despawn();

}

#[derive(Component)]
pub struct GameOverComp;

