use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::ecs::system::EntityCommands;
use bevy::math::{EulerRot, Vec2, Vec3};
use bevy::render::color::Color;
use bevy::transform::components::Transform;
use bevy_prototype_lyon::draw::DrawMode;
use bevy_prototype_lyon::draw::StrokeMode;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes::Line;
use heron::CollisionLayers;
use heron::CollisionShape;
use heron::PhysicMaterial;
use heron::RigidBody;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::world;

// 3.5 is a lot (since real wdth is the double of that).  Unfortunately,
// projectiles go throught thin bumpers when they have some speed, as physics
// system do not detect the collision :-(
const WIDTH: f32 = 3.5;

#[derive(Bundle)]
pub struct BumperBundle {
    body: RigidBody,
    shape: CollisionShape,
    layer: CollisionLayers,
    material: PhysicMaterial,
}

impl Default for BumperBundle {
    fn default() -> Self {
        BumperBundle {
            body: RigidBody::Static,
            shape: CollisionShape::Sphere { radius: 0.5 },
            layer: CollisionLayers::none()
                .with_group(world::Layer::Bumpers)
                .with_masks(&[world::Layer::Projectiles]),
            material: PhysicMaterial {
                friction: 0.01,
                density: 2.0,
                restitution: 0.9,
            },
        }
    }
}

#[derive(Component)]
pub struct Bumper {
    life: f32,
    initial_life: f32,
    material: PhysicMaterial,
    transform: Option<Transform>,
    angle: f32,
    half_length: f32,
    color: Color,
}

impl Bumper {
    pub fn new(t: BumperType) -> Bumper {
        let stats = t.get_stats();
        Bumper {
            half_length: stats.0,
            life: stats.1,
            initial_life: stats.1,
            color: stats.4,
            material: PhysicMaterial {
                friction: stats.3,
                density: 100.0,
                restitution: stats.2,
            },
            angle: 0.0,
            transform: None,
        }
    }

    pub fn create_random() -> Bumper {
        Self::new(rand::random())
    }

    fn body(&self) -> BumperBundle {
        let shape = CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-self.half_length, -WIDTH, 0.0),
                Vec3::new(-self.half_length, WIDTH, 0.0),
                Vec3::new(self.half_length, WIDTH, 0.0),
                Vec3::new(self.half_length, -WIDTH, 0.0),
            ],
            border_radius: None,
        };

        BumperBundle {
            shape: shape,
            material: self.material,
            ..Default::default()
        }
    }

    pub fn shape_bundle(&self) -> ShapeBundle {
        let (color, transform) = match self.transform {
            Some(transform) => (self.color, transform),
            None => (Color::GRAY, Transform::from_xyz(500.0, 200.0, 10.0)),
        };
        GeometryBuilder::build_as(
            &Line(
                Vec2::new(-self.half_length, 0.0),
                Vec2::new(self.half_length, 0.0),
            ),
            DrawMode::Stroke(StrokeMode::new(color, 2.0)),
            transform,
        )
    }

    pub fn take_damage(&mut self) {
        self.life -= 1.0;
    }

    pub fn fix(&mut self, mut entity: EntityCommands, transform: &Transform) {
        self.angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        self.transform = Some(*transform);
        entity
            .insert_bundle(self.shape_bundle())
            .insert_bundle(self.body());
    }
}

pub enum BumperType {
    Standard,
    Long,
    Bounciest,
    Hard,
    Wood,
}

impl Distribution<BumperType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BumperType {
        match rng.gen_range(0..=4) {
            0 => BumperType::Standard,
            1 => BumperType::Long,
            2 => BumperType::Bounciest,
            3 => BumperType::Hard,
            _ => BumperType::Wood,
        }
    }
}

impl BumperType {
    fn get_stats(&self) -> (f32, f32, f32, f32, Color) {
        match self {
            // Length, hits, bounciness, friction, color
            Self::Standard => (35.0, 30.0, 1.0, 0.1, Color::CYAN),
            Self::Long => (50.0, 30.0, 0.95, 0.2, Color::BLUE),
            Self::Bounciest => (40.0, 35.0, 1.3, 0.01, Color::PINK),
            Self::Hard => (35.0, 60.0, 0.9, 0.2, Color::RED),
            Self::Wood => (40.0, 50.0, 0.1, 0.5, Color::MAROON),
        }
    }
}
