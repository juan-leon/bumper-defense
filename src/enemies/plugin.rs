use bevy::app::{App, Plugin};

use crate::enemies::enemy;
use crate::enemies::spawner;

// TODO: add dificulty
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(spawner::Spawner::new(1.0)) // TODO: use difficulty
            .insert_resource(enemy::get_timer_resource())
            .add_system(spawner::spawn_enemy)
            .add_system(enemy::manage_enemy_movement)
            .add_system(enemy::shoot)
            .add_system(enemy::manage_enemy);
    }
}
