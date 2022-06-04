use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use heron::Gravity;

use crate::world::{landscape, systems};
use crate::world::GRAVITY;

// TODO: add dificulty
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(landscape::CLEAR_COLOR)
            .insert_resource(Gravity::from(Vec3::Y * -GRAVITY))
            .add_startup_system(landscape::create_floor)
            .add_system(systems::despawner);
    }
}

