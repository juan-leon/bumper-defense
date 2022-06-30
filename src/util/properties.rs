use bevy::ecs::component::Component;

// Those components act as transient properties

// Entity (enemy) is shooting (ir in position and has bullets)
#[derive(Component)]
pub struct Shooter;

// Entity hit the ground this framse
#[derive(Component)]
pub struct JustLanded;

// Entity exploded in this frame
#[derive(Component)]
pub struct Exploded;

// Bumper was activated (it rebounded a projectile i this frame)
#[derive(Component)]
pub struct BumperActivated;

// Entity should be despawn in clean up phase
#[derive(Component)]
pub struct Done;
