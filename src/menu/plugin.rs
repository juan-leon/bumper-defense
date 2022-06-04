use bevy::app::{App, Plugin};
use bevy::ecs::schedule::SystemSet;

use crate::game::AppState;
use crate::menu::systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(systems::setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(systems::menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(systems::cleanup_menu));
    }
}
