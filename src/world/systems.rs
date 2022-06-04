use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query};
use bevy::transform::components::Transform;
use heron::Velocity;

use crate::world::VISIBLE_WORLD_WIDTH;

const DESPAWN_MARGIN: f32 = 100.0;
const MAX_HEIGHT: f32 = VISIBLE_WORLD_WIDTH * 20.0;

pub fn despawner(
    mut commands: Commands,
    mut sprite_position: Query<(Entity, &Transform), With<Velocity>>,
) {
    for (entity, transform) in sprite_position.iter() {
        if transform.translation.y < -DESPAWN_MARGIN
            || transform.translation.x < -DESPAWN_MARGIN
            || transform.translation.x > VISIBLE_WORLD_WIDTH + DESPAWN_MARGIN
            || transform.translation.y > MAX_HEIGHT
        {
            commands.entity(entity).despawn();
        }
    }
}
