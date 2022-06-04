use bevy::prelude::*;
use core::f32::consts::PI;

use rand::{thread_rng, Rng};

use bevy::math::{const_vec3, Vec3};
use heron::prelude::*;
use heron::CollisionData;

use crate::world;


const PLAYER_POSITION: f32 = 400.0;  // FIXME move that to player module

#[derive(Bundle)]
pub struct ProjectileBundle {
    body: RigidBody,
    shape: CollisionShape,
    velocity: Velocity,
    rotation: RotationConstraints,
    layer: CollisionLayers,
    material: PhysicMaterial,
}

impl Default for ProjectileBundle {
    // TODO: create several projectile types.  Radius, bounces,
    // damage, restitution, sprite, CollisionShape, ... should vary
    // according to that.
    fn default() -> Self {
        ProjectileBundle {
            body: RigidBody::Dynamic,
            shape: CollisionShape::Sphere { radius: 3.5 },
            velocity: Velocity::from_linear(Vec3::ZERO),
            rotation: RotationConstraints::allow(),
            layer: CollisionLayers::none()
                .with_group(world::Layer::Projectiles)
                .with_masks(&[
                    world::Layer::World,
                    world::Layer::Bumpers,
                    world::Layer::Projectiles,
                ]),
            material: PhysicMaterial {
                friction: 0.2,
                density: 2.0,
                restitution: 0.5,
            }
        }
    }
}

pub enum LandEffect {
    Bounce,
    Explode,
}

#[derive(Component)]
pub struct Projectile {
    radius: f32,
    bounces: usize,
}

impl Projectile {
    pub fn new() -> Projectile {
        let mut rng = thread_rng();
        Projectile {
            radius: 3.7,
            bounces: thread_rng().gen_range(0..4),
        }
    }

    pub fn get_bundle(&self, origin: Vec3) -> ProjectileBundle {
        ProjectileBundle {
            shape: CollisionShape::Sphere { radius: self.radius },
            velocity: self.calculate_velocity(origin),
            ..default()
        }
    }

    pub fn sprite(&self, origin: Vec3, asset_server: &AssetServer) -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(origin.x, origin.y, 0.0),
                // FIXME: scale should be a function of orthographic projection
                // and sprite size
                scale: const_vec3!([0.046, 0.046, 1.0]),
                ..default()
            },
            texture: asset_server.load("ball2.png"),
            ..default()
        }
    }

    fn calculate_velocity(&self, origin: Vec3) -> Velocity {
        // We start with a random vertical velocity.  Another option could be a
        // random angle
        //
        // FIXME use another random distribution, like normal, to make it easier
        // to the player predict most trajectories
        let vertical_v: f32 = thread_rng().gen_range(80.0..600.0);
        // This is the time the projectile will hit the floor
        let time = (vertical_v + ((vertical_v * vertical_v) + 2.0 * origin.y * world::GRAVITY).sqrt()) / world::GRAVITY;
        // This is the horizontal velocity it needs to hit the player in that time
        let horizontal_v = (PLAYER_POSITION - origin.x) / time;
        Velocity::from_linear(Vec3::new(horizontal_v, vertical_v, 0.0 ))
    }

    pub fn touch_ground(&mut self) -> LandEffect {
        if self.bounces == 0 {
            LandEffect::Explode
        } else {
            self.bounces -= 1;
            LandEffect::Bounce
        }
    }
}
