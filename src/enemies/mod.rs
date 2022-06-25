pub use self::plugin::EnemyPlugin;

mod enemy;
mod explosion;
mod flasher;
mod plugin;
mod projectile;
mod spawner;
// FIXME the pub is needed because of BumperActivated; I should move collision
// detection elsewhere
pub mod systems;
