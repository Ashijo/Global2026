use bevy::prelude::{Commands, Component, Entity, Query, With};
#[derive(Component)]
pub struct LevelEntity;

pub fn despawn_entities<'a>(
    commands: &mut Commands,
    entities: impl Iterator<Item = Entity> + 'a,
) {
    for e in entities {
        commands.entity(e).despawn();
    }
}