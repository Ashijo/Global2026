use bevy::prelude::{Commands, Component, Entity};
#[derive(Component)]
pub struct LevelEntity;

pub fn despawn_entities<'a>(
    commands: &mut Commands,
    entities: impl Iterator<Item = Entity> + 'a,
) {
    for e in entities {
        commands.entity(e).despawn();
        println!("entity");
    }
}