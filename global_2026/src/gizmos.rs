use bevy::{color::palettes::css::*, prelude::*};
use crate::collision::Hitbox;
use crate::enemy::Detection;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                gizmo_hitbox,
                gizmo_detection,
            )
        );
    }
}


fn gizmo_hitbox(mut gizmos: Gizmos, hitbox_query: Query<(&Hitbox, &Transform)>) {
    for (hitbox, transform) in &hitbox_query {
        let min_x = transform.translation.x - hitbox.size.x / 2.0 + hitbox.offset.x;
        let max_x = transform.translation.x + hitbox.size.x / 2.0 + hitbox.offset.x;
        let min_y = transform.translation.y - hitbox.size.y / 2.0 + hitbox.offset.y;
        let max_y = transform.translation.y + hitbox.size.y / 2.0 + hitbox.offset.y;

        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(min_x, max_y), RED);
        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(max_x, min_y), RED);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(min_x, max_y), RED);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(max_x, min_y), RED);
    }
}



fn gizmo_detection(mut gizmos: Gizmos, detection_query: Query<(&Detection, &Transform)>) {
    for (detection, transform) in &detection_query {
        let min_x = transform.translation.x - detection.size.x / 2.0;
        let max_x = transform.translation.x + detection.size.x / 2.0;
        let min_y = transform.translation.y - detection.size.y / 2.0;
        let max_y = transform.translation.y + detection.size.y / 2.0;

        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(min_x, max_y), GREEN);
        gizmos.line_2d(Vec2::new(min_x, min_y), Vec2::new(max_x, min_y), GREEN);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(min_x, max_y), GREEN);
        gizmos.line_2d(Vec2::new(max_x, max_y), Vec2::new(max_x, min_y), GREEN);
    }
}
