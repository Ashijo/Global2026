use bevy::prelude::*;
use bevy::window::WindowEvent::KeyboardInput;
const SPEED:f32 = 300.0;

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(25.)),
            image: asset_server.load("test.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
pub fn player_update(keys: Res<ButtonInput<KeyCode>>,
                     mut query: Query<&mut Transform, With<Player>>,
                     time:Res<Time>)  {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyA){
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS){
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD){
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW){
        direction.y += 1.0;
    }
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    let delta = time.delta_secs();
    if let Ok(mut transform) = query.single_mut(){
        transform.translation.x += direction.x * SPEED * delta;
        transform.translation.y += direction.y * SPEED * delta;
    }
}
pub fn player_fixed_update() {
    println!("player fixed update")
}

#[derive(Component)]
pub struct Player;
