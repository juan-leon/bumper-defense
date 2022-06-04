use bevy::app::{App, Plugin};
use bevy::ecs::schedule::SystemSet;

use crate::game::AppState;
use crate::player::systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(systems::create_tower),
        );
    }
}
