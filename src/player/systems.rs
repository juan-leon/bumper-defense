use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query, ResMut};
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use bevy::ecs::query::With;
use bevy::ecs::system::QuerySingleError;
use bevy::math::Quat;

use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody};

use crate::player::bumper::Bumper;
use crate::player::tower::Tower;
use crate::world;

#[derive(Component)]
pub struct Floating;

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

pub fn spawn_or_place_bumper(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    bumper_q: Query<(&Transform, &Bumper, Entity), With<Floating>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    match bumper_q.get_single() {
        Ok((transform, bumper, entity)) => {
            // Maybe weird, since bumper is not aware of its fix state
            commands.entity(entity).remove::<Floating>();
            commands
                .entity(entity)
                .insert_bundle(bumper.fixed_bundle(transform));
        }
        Err(QuerySingleError::NoEntities(_)) => {
            let bumper = Bumper::new();
            commands
                .spawn_bundle(bumper.floating_bundle())
                .insert(bumper)
                .insert(Floating);
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Two floating bumpers at same time!")
        }
    }
}

pub fn move_bumper(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut bumper_q: Query<&mut Transform, With<Floating>>,
) {
    if bumper_q.is_empty() {
        return;
    }
    let mut transform = bumper_q.single_mut();
    let multiplier = if keys.pressed(KeyCode::LShift) {
        1.0
    } else if keys.pressed(KeyCode::LControl) {
        15.0
    } else {
        5.0
    };
    if keys.pressed(KeyCode::Up) {
        transform.translation.y += multiplier;
    } else if keys.pressed(KeyCode::Down) {
        transform.translation.y -= multiplier;
    }
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += multiplier;
    } else if keys.pressed(KeyCode::Left) {
        transform.translation.x -= multiplier;
    }

    if keys.pressed(KeyCode::A) {
        transform.rotate(Quat::from_rotation_z(0.012 * multiplier));
    } else if keys.pressed(KeyCode::D) {
        transform.rotate(Quat::from_rotation_z(-0.012 * multiplier));
    }
}
