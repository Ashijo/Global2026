use bevy::prelude::*;

pub fn enemy_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("img/enemy.png");

    // from_grid define spritsheet division ( tile_size: UVec2,
    //     columns: u32,
    //     rows: u32,
    //     padding: Option<UVec2>,
    //     offset: Option<UVec2>)
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 13, 1, Some(UVec2::splat(1)), Some(UVec2::splat(1)));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    let mut tr = Transform::from_scale(Vec3::splat(4.0));
    tr.translation = Vec3::new(150.0, 70.0, 1.0);


    commands.spawn((
        Enemy,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        tr,
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    ));

}
pub fn enemy_update() {

}
pub fn enemy_fixed_update(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }

}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

