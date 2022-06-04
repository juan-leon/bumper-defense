use bevy::core_pipeline::ClearColor;
use bevy::ecs::system::Commands;
use bevy::math::const_vec3;
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;

use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody};

use crate::world;

const BACKGROUND_COLOR: Color = Color::rgb(0.09, 0.15, 0.09);
const FLOOR_COLOR: Color = Color::rgb(0.45, 0.75, 0.45);
pub const CLEAR_COLOR: ClearColor = ClearColor(BACKGROUND_COLOR);

pub const FLOOR_HEIGHT: f32 = 20.0;

pub fn create_floor(mut commands: Commands) {
    let layer = world::Layer::World;
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([
                    world::VISIBLE_WORLD_WIDTH / 2.0,
                    FLOOR_HEIGHT,
                    layer.to_z()
                ]),
                scale: const_vec3!([world::VISIBLE_WORLD_WIDTH, FLOOR_HEIGHT, 0.0]),
                ..Default::default()
            },
            sprite: Sprite {
                color: FLOOR_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: const_vec3!([world::VISIBLE_WORLD_WIDTH / 2.0, FLOOR_HEIGHT / 2.0, 0.0]),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 100.0,
            restitution: 0.5,
        })
        .insert(
            CollisionLayers::none()
                .with_group(layer)
                .with_mask(world::Layer::Projectiles),
        );
}
