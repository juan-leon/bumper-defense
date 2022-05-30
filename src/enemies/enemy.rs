use bevy::prelude::*;

use rand::{thread_rng, Rng};

use bevy::math::{const_vec3, Vec3};
use heron::prelude::*;

use crate::enemies::spawner::Spawner;
use crate::world;

const ENEMY_SIDE: f32 = 20.0;
const ENEMY_SIZE: Vec3 = const_vec3!([ENEMY_SIDE, ENEMY_SIDE, 0.0]);
const COLLISION_SHAPE: CollisionShape = CollisionShape::Cuboid {
    half_extends: const_vec3!([ENEMY_SIDE / 2.0, ENEMY_SIDE / 2.0, 0.0]),
    border_radius: None,
};

const MAX_LIFE: f32 = 100.0;
const MAX_BULLETS: usize = 10;

#[derive(Component)]
pub struct Enemy {
    id: usize,
    life: f32,
    target_x: f32,
    bullets: usize,
}

#[derive(Component)]
pub struct Shooter;

impl Enemy {
    pub fn new(id: usize) -> Enemy {
        let mut rng = thread_rng();
        Enemy {
            id: id,
            life: MAX_LIFE,
            target_x: rng.gen_range(-600.0..900.0),
            bullets: MAX_BULLETS,
        }
    }

    fn get_bundle(&self) -> SpriteBundle {
        let mut rng = thread_rng();
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-30.0, rng.gen_range(100.0..700.0), 0.0),
                scale: ENEMY_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        }
    }

    pub fn spawn(id: usize, mut commands: Commands) {
        let enemy = Enemy::new(id);
        commands
            .spawn_bundle(enemy.get_bundle())
            .insert(COLLISION_SHAPE)
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Velocity::from_linear(Vec3::X * 100.0))
            .insert(PhysicMaterial {
                friction: 1.0,
                density: 10.0,
                restitution: 0.5,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::Enemies)
                    .with_masks(&[world::Layer::World, world::Layer::Enemies]),
            )
            .insert(enemy);
    }

    pub fn shoot(&mut self) {
        self.bullets -= 1;
    }
}

pub fn manage_enemy(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    mut enemy: Query<(&Enemy, &mut Transform, Entity)>,
) {
    for (e, transform, entity) in enemy.iter() {
        if transform.translation.x > 1250.0 {
            commands.entity(entity).despawn();
            spawner.despawn();
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    mut enemy: Query<(&mut Enemy, &mut Transform, Entity), With<Shooter>>,
) {
    for (mut e, transform, entity) in enemy.iter_mut() {
        if e.bullets < 1 {
            commands.entity(entity).despawn();
            continue;
        }
        commands.entity(entity).remove::<Velocity>();
        e.shoot();
        let mut rng = thread_rng();
        commands
            .spawn_bundle((
                Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                )),
                GlobalTransform::default(),
            ))
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 11.0 })
            .insert(Velocity::from_linear(Vec3::X * rng.gen_range(50.0..150.0)))
            .insert(PhysicMaterial {
                friction: rng.gen_range(0.1..10.0),
                density: rng.gen_range(0.5..100.0),
                restitution: rng.gen_range(0.1..0.95),
            })
            .insert(RotationConstraints::allow())
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::Projectiles)
                    .with_masks(&[world::Layer::World, world::Layer::Bumpers]),
            );
    }
}

// d = V₀ * cos(α) * [V₀ * sin(α) + √((V₀ * sin(α))² + 2 * g * h)] / g

pub fn manage_enemy_movement(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    mut enemy: Query<(&Enemy, &mut Transform, &mut Velocity, Entity)>,
) {
    for (e, transform, mut velocity, entity) in enemy.iter_mut() {
        if velocity.linear == Vec3::ZERO {
            commands.entity(entity).remove::<Velocity>();
            commands.entity(entity).insert(Shooter);
        } else if transform.translation.x > e.target_x {
            velocity.linear /= Vec3::ONE * 1.03;
            if velocity.linear.max_element() < 0.05 {
                velocity.linear = Vec3::ZERO;
            }
        }
    }
}
