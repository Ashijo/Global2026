use bevy::prelude::*;

pub const STUN_DURATION: f32 = 2.0;

#[derive(Component)]
pub struct Stunned {
    pub timer: Timer,
}

// ticks and removes stun after timer
pub fn stun_update(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Stunned)>) {
    for (entity, mut stunned) in query.iter_mut() {
        stunned.timer.tick(time.delta());

        if stunned.timer.just_finished() {
            commands.entity(entity).remove::<Stunned>();
        }
    }
}
