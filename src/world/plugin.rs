use bevy::app::{App, Plugin};
use bevy::ecs::schedule::SystemSet;
use bevy::math::Vec3;
use heron::Gravity;

use crate::game::AppState;
use crate::world::GRAVITY;
use crate::world::{landscape, systems};

// TODO: add dificulty
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(landscape::CLEAR_COLOR)
            .insert_resource(Gravity::from(Vec3::Y * -GRAVITY))
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(landscape::create_floor),
            )
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(systems::despawner));
    }
}
