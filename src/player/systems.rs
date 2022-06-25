use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::system::QuerySingleError;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::input::keyboard::KeyCode;
use bevy::input::{ElementState, Input};
use bevy::math::Quat;
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;
use bevy::{
    input::mouse::{MouseButtonInput, MouseWheel},
    prelude::*,
    window::CursorMoved,
};

use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody};

use crate::enemies::systems::BumperActivated;
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
    keys: Res<Input<KeyCode>>,
    mut bumper_q: Query<(&Transform, &mut Bumper, Entity), With<Floating>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    match bumper_q.get_single_mut() {
        Ok((transform, mut bumper, entity)) => {
            fix_bumper(bumper.as_mut(), commands.entity(entity), *transform);
        }
        Err(QuerySingleError::NoEntities(_)) => {
            let bumper = Bumper::create_random();
            commands
                .spawn_bundle(bumper.shape_bundle())
                .insert(bumper)
                .insert(Floating);
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Two floating bumpers at same time!")
        }
    }
}

pub fn move_bumper(keys: Res<Input<KeyCode>>, mut bumper_q: Query<&mut Transform, With<Floating>>) {
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

// FIXME: do I need/want this?
pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::W) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

pub fn move_bumper_with_mouse(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    keys: Res<Input<KeyCode>>,
    mut bumper_q: Query<(&mut Transform, &mut Bumper, Entity), With<Floating>>,
) {
    if bumper_q.is_empty() {
        return;
    }
    let (mut transform, mut bumper, entity) = bumper_q.single_mut();
    let multiplier = if keys.pressed(KeyCode::LShift) {
        2.0
    } else if keys.pressed(KeyCode::LControl) {
        10.0
    } else {
        5.0
    };
    for event in mouse_wheel_events.iter() {
        transform.rotate(Quat::from_rotation_z(0.016 * multiplier * event.y));
    }
    if let Some(event) = cursor_moved_events.iter().last() {
        if let Some(window) = windows.get_primary() {
            transform.translation.x =
                event.position.x * world::VISIBLE_WORLD_WIDTH / window.width();
            transform.translation.y =
                event.position.y * world::VISIBLE_WORLD_HEIGHT / window.height();
        }
    }
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Pressed {
            fix_bumper(bumper.as_mut(), commands.entity(entity), *transform);
        }
        break;
    }
}

pub fn activated_system(
    mut commands: Commands,
    mut bumper_q: Query<(&mut Bumper, Entity), With<BumperActivated>>,
) {
    for (mut bumper, entity) in bumper_q.iter_mut() {
        commands.entity(entity).remove::<BumperActivated>();
        bumper.take_damage();
    }
}

fn fix_bumper(bumper: &mut Bumper, mut entity: EntityCommands, transform: Transform) {
    entity.remove::<Floating>();
    bumper.fix(entity, &transform);
}
