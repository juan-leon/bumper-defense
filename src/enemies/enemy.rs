use bevy::prelude::*;
use core::f32::consts::PI;

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
const MAX_BULLETS: usize = 25;

pub struct ShootTimer(Timer);

#[derive(Component)]
pub struct Enemy {
    id: usize,
    life: f32,
    target_x: f32,
    bullets: usize,
}

#[derive(Component)]
pub struct Shooter;

pub fn get_timer_resource() -> ShootTimer {
    ShootTimer(Timer::from_seconds(1.0, true))
}

fn calculate_velocity2(translation: Vec3) -> Velocity {
    let mut rng = thread_rng();
    let mut angle: f32 = PI / rng.gen_range(3.0..6.0);
    let x_force = 200.0 - translation.x;
    Velocity::from_linear(Vec3::new(
        x_force * angle.sin(),
        x_force.abs() * angle.cos(),
        0.0
    ))
}


fn calculate_velocity3(translation: Vec3) -> Velocity {
    // t = 2 * V₀ * sin(α) / g  ==> t = 2 * V₀ * 1 / g
    let mut rng = thread_rng();
    let mut angle: f32 = PI / rng.gen_range(3.0..6.0);
    let x_force = 200.0 - translation.x;
    let distance = 200.0 - translation.x;

    let vertical: f32 = rng.gen_range(80.0..600.0);
    let t = 2.0 * vertical / 600.0;

    let horizontal = distance / t;

    // t = 2 * V₀ * sin(α) / g  ==> t = 2 * V₀ * 1 / g
    //println!("H {} V {]")


    Velocity::from_linear(Vec3::new(
        horizontal,
        vertical,
        0.0
    ))
}


fn calculate_velocity(translation: Vec3) -> Velocity {
    // t = 2 * V₀ * sin(α) / g  ==> t = 2 * V₀ * 1 / g
    let mut rng = thread_rng();
    let distance = 200.0 - translation.x;

    let vertical: f32 = rng.gen_range(80.0..600.0);
    let g = 600.0;


    let t = (vertical + ((vertical * vertical) + 2.0 * translation.y * g).sqrt()) / g;

    let horizontal = distance / t;

    // t = 2 * V₀ * sin(α) / g  ==> t = 2 * V₀ * 1 / g
    //println!("H {} V {]")


    Velocity::from_linear(Vec3::new(
        horizontal,
        vertical,
        0.0
    ))
}



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
                translation: Vec3::new(-30.0, rng.gen_range(100.0..560.0), 0.0),
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
    time: Res<Time>,
    mut shoot_timer: ResMut<ShootTimer>,
    asset_server: Res<AssetServer>,
    mut enemy: Query<(&mut Enemy, &mut Transform, Entity), With<Shooter>>,
) {
    if !shoot_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for (mut e, transform, entity) in enemy.iter_mut() {
        if e.bullets < 1 {
            commands.entity(entity).despawn();
            continue;
        }
        commands.entity(entity).remove::<Velocity>();
        e.shoot();
        let mut rng = thread_rng();
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                    scale: const_vec3!([0.046, 0.046, 1.0]),
                    ..default()
                },
                texture: asset_server.load("ball2.png"),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 3.7 })
            .insert(calculate_velocity(transform.translation))
            .insert(PhysicMaterial {
                friction: rng.gen_range(0.1..0.2),
                density: 2.0,
                restitution: rng.gen_range(0.1..0.95),
            })
            .insert(RotationConstraints::allow())
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::Projectiles)
                    .with_masks(&[
                        world::Layer::World,
                        world::Layer::Bumpers,
                        world::Layer::Projectiles,
                    ]),
            );
    }
}

// d = V₀ * cos(α) * [V₀ * sin(α) + √((V₀ * sin(α))² + 2 * g * h)] / g

// t = 2 * V₀ * sin(α) / g  ==> t = 2 * V₀ * 1 / g

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
