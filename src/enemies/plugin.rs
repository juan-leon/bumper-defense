use bevy::app::{App, Plugin};
use bevy::ecs::schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet};

use crate::enemies::explosion;
use crate::enemies::spawner;
use crate::enemies::systems;
use crate::game::AppState;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct PreCollisionDetection;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct CollisionDetection;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct PostCollisionDetection;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct CleanUp;

// TODO: add dificulty
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(spawner::Spawner::new(1.0)) // TODO: use difficulty
            .insert_resource(explosion::ActiveExplosions::new())
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(PreCollisionDetection)
                    .with_system(systems::spawn_enemy)
                    .with_system(systems::despawn_enemy)
                    .with_system(systems::manage_enemy_movement)
                    .with_system(systems::shoot),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(CollisionDetection)
                    .after(PreCollisionDetection)
                    .with_system(systems::collision_dectector),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(PostCollisionDetection)
                    .after(CollisionDetection)
                    .with_system(systems::landed_system)
                    .with_system(systems::projectile_expired.after(systems::landed_system))
                    .with_system(systems::exploded_system.after(systems::projectile_expired))
                    .with_system(systems::damage_enemy_system.after(systems::exploded_system))
                    .with_system(systems::damage_tower_system.after(systems::exploded_system)),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(CleanUp)
                    .after(PostCollisionDetection)
                    .with_system(systems::despawn_lifebar)
                    .with_system(systems::update_lifebar.after(systems::despawn_lifebar))
                    .with_system(systems::update_tower_lifebar.after(systems::despawn_lifebar))
                    .with_system(systems::manage_flasher)
                    .with_system(systems::despawner),
            );
    }
}
