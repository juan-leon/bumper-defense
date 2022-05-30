use bevy::app::{App, Plugin};
use bevy::ecs::system::{Query, Commands};
use bevy::ecs::query::With;
use bevy::ecs::entity::Entity;
use bevy::transform::components::Transform;
use heron::Velocity;

use crate::world::landscape;
use crate::world::VISIBLE_WORLD_WIDTH;

const DESPAWN_MARGIN: f32 = 100.0;
const MAX_HEIGHT: f32 = VISIBLE_WORLD_WIDTH * 20.0;


// TODO: add dificulty
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(landscape::CLEAR_COLOR)
            .add_startup_system(landscape::create_floor)
            .add_system(despawner);
    }
}

fn despawner(
    mut commands: Commands,
    mut sprite_position: Query<(Entity, &Transform), With<Velocity>>,
) {
    for (entity, transform) in sprite_position.iter() {
        if (transform.translation.y < -DESPAWN_MARGIN
            || transform.translation.x < -DESPAWN_MARGIN
            || transform.translation.x > VISIBLE_WORLD_WIDTH + DESPAWN_MARGIN
            || transform.translation.y > MAX_HEIGHT
        ) {
            commands.entity(entity).despawn();
        }
    }
}
