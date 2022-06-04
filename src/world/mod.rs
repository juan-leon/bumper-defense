pub use self::landscape::FLOOR_HEIGHT;
pub use self::plugin::WorldPlugin;

pub mod landscape;
mod plugin;
mod systems;

use bevy::ecs::system::Commands;
use bevy::render::camera::{OrthographicCameraBundle, ScalingMode};
use bevy::window::WindowDescriptor;

use heron::PhysicsLayer;

pub const GRAVITY: f32 = 600.;
const ASPECT_RATIO: f32 = 9.0 / 16.0;
const VISIBLE_WORLD_WIDTH: f32 = 1000.0;
const VISIBLE_WORLD_HEIGHT: f32 = VISIBLE_WORLD_WIDTH * ASPECT_RATIO;

#[derive(PhysicsLayer)]
pub enum Layer {
    World,
    Player,
    Enemies,
    Bumpers,
    Projectiles,
    Trigger, // Anything that makes projectiles to explode upon contact
    Explosions,
}

impl Layer {
    pub fn to_z(&self) -> f32 {
        match self {
            Layer::World => 0.0,
            Layer::Player => 1.0,
            Layer::Enemies => 2.0,
            Layer::Bumpers => 3.0,
            Layer::Projectiles => 4.0,
            Layer::Trigger => 0.0, // Irrelevant
            Layer::Explosions => 5.0,
        }
    }
}

pub fn create_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.right = VISIBLE_WORLD_WIDTH;
    camera.orthographic_projection.left = 0.0;
    camera.orthographic_projection.top = VISIBLE_WORLD_HEIGHT;
    camera.orthographic_projection.bottom = 0.0;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}

pub fn create_window(width: f32) -> WindowDescriptor {
    WindowDescriptor {
        title: "Bumper defense".to_string(),
        width: width,
        height: width * ASPECT_RATIO,
        resizable: false,
        ..Default::default()
    }
}
