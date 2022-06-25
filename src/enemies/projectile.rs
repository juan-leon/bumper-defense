use bevy::asset::AssetServer;
use bevy::core::Timer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::sprite::SpriteBundle;
use bevy::transform::components::Transform;
use bevy::utils::Duration;
use core::ops::Range;
use heron::{
    CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints, Velocity,
};
use rand::{thread_rng, Rng};

use crate::enemies::explosion::{Explosion, ExplosionType};
use crate::player::tower;
use crate::world;

const PLAYER_POSITION: f32 = tower::POSITION;
const VERTICAL_V_RANGE: Range<f32> = 80.0..400.0;
// FIXME: move to asset related mod
const SPRITE_SIZE: f32 = 128.0;
const MAX_TTL: f32 = 8.0;

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
                    world::Layer::Player,
                    world::Layer::World,
                    world::Layer::Enemies,
                    world::Layer::Bumpers,
                    world::Layer::Projectiles,
                ]),
            material: PhysicMaterial {
                friction: 0.2,
                density: 2.0,
                restitution: 0.5,
            },
        }
    }
}

pub enum LandEffect {
    Bounce,
    Explode,
}

#[derive(Component)]
pub struct Projectile {
    origin: Vec3,
    radius: f32,
    bounces: usize,
    explosions: usize,
    explosion_type: ExplosionType,
    timer: Timer,
}

impl Projectile {
    pub fn new(origin: Vec3, t: ProjectileType) -> Projectile {
        let stats = t.get_stats();
        // FIXME: physic material & shape
        Projectile {
            origin,
            radius: stats.0,
            bounces: stats.1,
            explosions: stats.2,
            explosion_type: stats.4,
            timer: Timer::from_seconds(MAX_TTL, false),
        }
    }

    pub fn get_bundle(&self) -> ProjectileBundle {
        ProjectileBundle {
            shape: CollisionShape::Sphere {
                radius: self.radius,
            },
            velocity: self.calculate_velocity(),
            ..Default::default()
        }
    }

    pub fn sprite(&self, asset_server: &AssetServer) -> SpriteBundle {
        let scale = 2.0 * self.radius / SPRITE_SIZE;
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    self.origin.x,
                    self.origin.y,
                    world::Layer::Projectiles.to_z(),
                ),
                scale: Vec3::new(scale, scale, 0.0),
                ..Default::default()
            },
            texture: asset_server.load("p-hot-red.png"),
            ..Default::default()
        }
    }

    fn calculate_velocity(&self) -> Velocity {
        // We start with a random vertical velocity.  Another approach could be a
        // random angle
        //
        // FIXME use another random distribution, like normal, to make it easier
        // to the player predict most trajectories
        let vertical_v: f32 = thread_rng().gen_range(VERTICAL_V_RANGE);
        // This is the time the projectile will hit the floor
        let time = (vertical_v
            + ((vertical_v * vertical_v) + 2.0 * self.origin.y * world::GRAVITY).sqrt())
            / world::GRAVITY;
        // This is the horizontal velocity it needs to hit the player in that time
        let horizontal_v = (PLAYER_POSITION - self.origin.x) / time;
        Velocity::from_linear(Vec3::new(horizontal_v, vertical_v, 0.0))
    }

    pub fn touch_ground(&mut self) -> LandEffect {
        if self.bounces == 0 {
            LandEffect::Explode
        } else {
            self.bounces -= 1;
            LandEffect::Bounce
        }
    }

    pub fn explode(&mut self, location: Vec3) -> Option<Explosion> {
        if self.explosions > 0 {
            self.explosions -= 1;
            Some(Explosion::new(location, self.explosion_type))
        } else {
            None
        }
    }

    pub fn done(&self) -> bool {
        self.explosions == 0
    }

    pub fn tick(&mut self, duration: Duration, location: Vec3) -> Option<Explosion> {
        if self.timer.tick(duration).finished() {
            self.explosions = 0;
            Some(Explosion::new(location, self.explosion_type))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProjectileType {
    WeakShot,
    BigShot,
    HugeShot,
    HotShot,
    MiniShot,
}

impl ProjectileType {
    fn get_stats(&self) -> (f32, usize, usize, bool, ExplosionType) {
        match self {
            // Radius, Bounces, Explosions, Square, Explosion
            Self::WeakShot => (3.4, 1, 1, false, ExplosionType::WeakShot),
            Self::BigShot => (3.6, 2, 1, false, ExplosionType::BigShot),
            Self::HugeShot => (3.8, 3, 1, false, ExplosionType::HugeShot),
            Self::HotShot => (3.5, 4, 2, false, ExplosionType::HotShot),
            Self::MiniShot => (3.0, 4, 4, false, ExplosionType::MiniShot),
        }
    }
}
