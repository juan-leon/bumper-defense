use bevy::prelude::*;
use core::f32::consts::PI;

use rand::{thread_rng, Rng};

use bevy::math::{const_vec3, Vec3};
use heron::prelude::*;
use heron::CollisionData;

use crate::enemies::spawner::Spawner;
use crate::enemies::projectile::{Projectile, LandEffect};
use crate::world;

const ENEMY_SIDE: f32 = 20.0;
const ENEMY_SIZE: Vec3 = const_vec3!([ENEMY_SIDE, ENEMY_SIDE, 0.0]);
const COLLISION_SHAPE: CollisionShape = CollisionShape::Cuboid {
    half_extends: const_vec3!([ENEMY_SIDE / 2.0, ENEMY_SIDE / 2.0, 0.0]),
    border_radius: None,
};

const MAX_LIFE: f32 = 100.0;
const MAX_BULLETS: usize = 10;

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


#[derive(Component)]
pub struct Landed;


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
            target_x: rng.gen_range(50.0..900.0),
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

    pub fn shoot(&mut self) -> Projectile {
        self.bullets -= 1;
        Projectile::new()
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
    mut enemy: Query<(&mut Enemy, &Transform, Entity), With<Shooter>>,
) {
    if !shoot_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for (mut e, transform, entity) in enemy.iter_mut() {
        if e.bullets < 1 {
            commands.entity(entity).despawn();
            continue;
        }
        let projectile = e.shoot();
        commands.spawn_bundle(
            projectile.sprite(transform.translation, asset_server.as_ref())
        )
            .insert_bundle(projectile.get_bundle(transform.translation))
            .insert(projectile);
    }
}

pub fn manage_enemy_movement(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    mut enemy: Query<(&Enemy, &Transform, &mut Velocity, Entity)>,
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


pub fn collision_tester(
    mut commands: Commands,
    mut enemy: Query<&Collisions>,
) {
    for (collisions) in enemy.iter_mut() {
        println!("in outer loop");
        if !collisions.is_empty() {
            println!("Touching!");
            for e in collisions.iter() {
                println!("Inner loop {:?}", e);
            }
        }
    }
}

pub fn collision_tester4(
    mut commands: Commands,
    mut enemy: Query<(&Collisions, &mut Transform, Entity), With<Projectile>>,
) {
    for (collisions, transform, entity) in enemy.iter_mut() {
        println!("Proj {:?}", entity);
        if !collisions.is_empty() {
            println!("Touching!");
            for e in collisions.iter() {
                println!("Proj {:?} touches {:?} at {:?}", entity, e, transform.translation);
            }
        }
    }
}


pub fn collision_tester3(
    mut commands: Commands,
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut enemy: Query<(&mut Transform, Entity), With<Projectile>>,
) {
    for (transform, entity) in enemy.iter_mut() {
        println!("Proj {:?}", entity);
    }
    for event in collision_events.iter() {
        // println!("Event {:?}", event);
        let (data1, data2) = event.clone().data();
        // println!("Data {:?} {:?}", data1, data2);
        let (entity1, entity2) = (data1.rigid_body_entity(), data2.rigid_body_entity());
        println!("Entities {:?} {:?}", entity1, entity2);
    }
}


pub fn collision_dectector(
    mut commands: Commands,
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
) {
    for event in collision_events.iter() {
        if event.is_started() {
            // println!("Event {:?}", event);
            let (data1, data2) = event.clone().data();
            let (layers1, layers2) = (data1.collision_layers(), data2.collision_layers());
            let mut projectile: Option<CollisionData> = None;
            if layers1.contains_group(world::Layer::Projectiles) && layers2.contains_group(world::Layer::World) {
                projectile = Some(data1);
            } else if layers2.contains_group(world::Layer::Projectiles) && layers1.contains_group(world::Layer::World) {
                projectile = Some(data2);
            }
            match projectile {
                Some(data) => {
                    commands.entity(data.rigid_body_entity()).insert(Landed);
                },
                None => (),
            }
        }
    }
}

pub fn landed(data: CollisionData) -> bool {
    let layers = data.collision_layers();
    layers.contains_group(world::Layer::Projectiles) && layers.contains_mask(world::Layer::World)
}

pub fn landed_system(
    mut commands: Commands,
    mut enemy: Query<(&Transform, &mut Projectile, Entity), With<Landed>>,
) {
    for (transform, mut projectile, entity) in enemy.iter_mut() {
        match projectile.touch_ground() {
            LandEffect::Bounce => {
                commands.entity(entity).remove::<Landed>();
            }
            LandEffect::Explode => {
                commands.entity(entity).despawn();
            }
        }
    }
}
