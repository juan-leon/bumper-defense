use bevy::ecs::system::Commands;
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;

use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody};

use crate::player::tower::Tower;
use crate::world;

const COLOR: Color = Color::rgb(0.25, 0.25, 0.75);

pub fn create_tower(mut commands: Commands) {
    let tower = Tower::new();
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: tower.translation(),
                scale: tower.scale(),
                ..Default::default()
            },
            sprite: Sprite {
                color: COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: tower.shape(),
            border_radius: Some(3.0),
        })
        .insert(PhysicMaterial {
            friction: 5.0,
            density: 100.0,
            restitution: 0.5,
        })
        .insert(
            CollisionLayers::none()
                .with_groups(&[world::Layer::Player, world::Layer::Trigger])
                .with_masks(&[world::Layer::Projectiles, world::Layer::Explosions]),
        )
        .insert(tower);
}
