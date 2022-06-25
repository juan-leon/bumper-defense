use bevy::core::Timer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::math::{const_vec3, Vec2, Vec3};
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;
use bevy::utils::Duration;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, Velocity};
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

use crate::enemies::projectile::{Projectile, ProjectileType};
use crate::util::lifebar::Lifebar;
use crate::world;

const ENEMY_SIDE: f32 = 20.0;
const ENEMY_SIZE: Vec3 = const_vec3!([ENEMY_SIDE, ENEMY_SIDE, 0.0]);
const COLLISION_SHAPE: CollisionShape = CollisionShape::Cuboid {
    half_extends: const_vec3!([ENEMY_SIDE / 2.0, ENEMY_SIDE / 2.0, 0.0]),
    border_radius: None,
};

const INITIAL_VELOCITY: f32 = 100.0;
const DECELERATION: f32 = 0.97;

pub enum ShootResult {
    Fire(Projectile),
    GoAway(Velocity),
    Pass,
}

enum EnemyDirection {
    Right,
    Left,
}

impl Distribution<EnemyDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyDirection {
        match rng.gen_range(0..=1) {
            0 => EnemyDirection::Right,
            _ => EnemyDirection::Left,
        }
    }
}

impl EnemyDirection {
    fn velocity(&self) -> Velocity {
        match self {
            Self::Right => Velocity::from_linear(Vec3::X * INITIAL_VELOCITY),
            Self::Left => Velocity::from_linear(Vec3::X * -INITIAL_VELOCITY),
        }
    }

    fn escape_velocity(&self) -> Velocity {
        match self {
            Self::Right => Velocity::from_linear(Vec3::X * -INITIAL_VELOCITY),
            Self::Left => Velocity::from_linear(Vec3::X * INITIAL_VELOCITY),
        }
    }

    fn start(&self) -> Vec3 {
        let mut rng = thread_rng();
        let height = rng.gen_range(100.0..560.0); // FIXME use constants
        match self {
            Self::Right => Vec3::new(-30.0, height, 0.0),
            Self::Left => Vec3::new(1030.0, height, 0.0), // FIXME use constants
        }
    }

    fn escaped(&self, x: f32) -> bool {
        match self {
            // FIXME: constants
            Self::Right => x < -30.0,
            Self::Left => x > 1030.0,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    body: RigidBody,
    shape: CollisionShape,
    velocity: Velocity,
    layer: CollisionLayers,
    material: PhysicMaterial,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            body: RigidBody::KinematicVelocityBased,
            shape: COLLISION_SHAPE,
            velocity: Velocity::from_linear(Vec3::X * INITIAL_VELOCITY),
            layer: CollisionLayers::none()
                .with_groups(&[world::Layer::Enemies, world::Layer::Trigger])
                .with_masks(&[
                    world::Layer::World,
                    world::Layer::Enemies,
                    world::Layer::Projectiles,
                    world::Layer::Explosions,
                ]),
            material: PhysicMaterial {
                friction: 1.0,
                density: 10.0,
                restitution: 0.5,
            },
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    life: f32,
    initial_life: f32,
    target_x: f32,
    direction: EnemyDirection,
    bullets: usize,
    location: Vec3,
    timer: Timer,
    projectile_type: ProjectileType,
    color: Color,
}

impl Enemy {
    pub fn new(t: EnemyType) -> Enemy {
        let mut rng = thread_rng();
        let stats = t.get_stats();
        Enemy {
            life: stats.0,
            initial_life: stats.0,
            bullets: stats.1,
            timer: Timer::from_seconds(stats.2, true),
            projectile_type: stats.3,
            color: stats.4,
            target_x: rng.gen_range(50.0..410.0), // FIXME use constant
            direction: rand::random(),
            location: Vec3::ZERO,
        }
    }

    pub fn create_random() -> Enemy {
        Self::new(rand::random())
    }

    pub fn sprite(&self) -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation: self.direction.start(),
                scale: ENEMY_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: self.color,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn get_bundle(&self) -> EnemyBundle {
        EnemyBundle {
            velocity: self.direction.velocity(),
            ..Default::default()
        }
    }

    pub fn ready(&mut self, time: Duration) -> bool {
        self.timer.tick(time).just_finished()
    }

    pub fn shoot(&mut self, origin: Vec3) -> ShootResult {
        if self.is_dead() {
            ShootResult::Pass
        } else if self.bullets > 0 {
            self.bullets -= 1;
            let proj_origin = origin
                + match self.direction {
                    // FIXME: The 4.0 is for clearance, until I have a better
                    // collision shape for enemies
                    EnemyDirection::Right => {
                        Vec3::new(4.0 + (ENEMY_SIDE / 2.0), ENEMY_SIDE / 2.0, 0.0)
                    }
                    EnemyDirection::Left => {
                        Vec3::new(-4.0 - (ENEMY_SIDE / 2.0), ENEMY_SIDE / 2.0, 0.0)
                    }
                };
            ShootResult::Fire(Projectile::new(proj_origin, self.projectile_type))
        } else {
            ShootResult::GoAway(self.direction.escape_velocity())
        }
    }

    pub fn adjust_velocity(&mut self, coordinates: Vec3, velocity: Vec3) -> Option<Vec3> {
        self.location = coordinates;
        let should_stop = self.bullets > 0
            && match self.direction {
                EnemyDirection::Right => coordinates.x > self.target_x,
                EnemyDirection::Left => 1000.0 - coordinates.x > self.target_x, // FIXME constant
            };
        if should_stop {
            let new_velocity = velocity * DECELERATION;
            if new_velocity.x < 0.08 {
                // FIXME: store position here???
                None
            } else {
                Some(new_velocity)
            }
        } else {
            Some(velocity)
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.life -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }

    // Returns a pseudo collision shape in the form of (center, radius) tuple
    pub fn as_circle(&self) -> (Vec2, f32) {
        (self.location.truncate(), 1.05 * ENEMY_SIDE / 2.0)
    }

    pub fn done(&self) -> bool {
        self.is_dead() || (self.bullets <= 0 && self.direction.escaped(self.location.x))
    }

    pub fn get_lifebar(&self) -> Option<Lifebar> {
        let relative_position = Vec3::new(-ENEMY_SIDE / 2.0, (ENEMY_SIDE + 4.0) / 2.0, 0.0);
        if self.life > 0.0 {
            Some(Lifebar::new(
                self.location + relative_position,
                self.life / self.initial_life,
            ))
        } else {
            None
        }
    }
}

pub enum EnemyType {
    Weak,
    Medium,
    Hard,
    Demon,
    Bouncer,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        match rng.gen_range(0..=4) {
            0 => EnemyType::Weak,
            1 => EnemyType::Medium,
            2 => EnemyType::Hard,
            3 => EnemyType::Demon,
            _ => EnemyType::Bouncer,
        }
    }
}

impl EnemyType {
    fn get_stats(&self) -> (f32, usize, f32, ProjectileType, Color) {
        match self {
            // Life, bullets, shoot-delay, projectile, color
            Self::Weak => (75.0, 50, 1.5, ProjectileType::WeakShot, Color::CYAN),
            Self::Medium => (85.0, 50, 2.0, ProjectileType::BigShot, Color::GREEN),
            Self::Hard => (95.0, 50, 2.2, ProjectileType::HugeShot, Color::ORANGE),
            Self::Demon => (100.0, 100, 1.5, ProjectileType::HotShot, Color::RED),
            Self::Bouncer => (110.0, 200, 0.8, ProjectileType::MiniShot, Color::PURPLE),
        }
    }
}
