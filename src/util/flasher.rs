use bevy::core::Timer;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::render::color::Color;
use bevy::transform::components::Transform;
use bevy::utils::Duration;

use bevy_prototype_lyon::draw::DrawMode;
use bevy_prototype_lyon::draw::FillMode;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;

#[derive(Component)]
pub struct Flasher {
    location: Vec2,
    radius: f32,
    color: Color,
    timer: Timer,
}

impl Flasher {
    pub fn new(location: Vec2, t: FlasherType) -> Flasher {
        let stats = t.get_stats();
        Flasher {
            location,
            radius: stats.0,
            color: stats.1,
            timer: Timer::from_seconds(stats.2, false),
        }
    }

    pub fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            radius: self.get_radius(),
            center: self.location,
        }
    }

    pub fn get_bundle(&self) -> ShapeBundle {
        GeometryBuilder::build_as(
            &self.get_shape(),
            DrawMode::Fill(FillMode::color(self.get_color())),
            Transform::from_xyz(0.0, 0.0, 10.0), // FIXME: constant
        )
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    fn get_color(&self) -> Color {
        let alpha: f32 = self.timer.percent_left().clamp(0.0, 0.8).powi(4);
        *self.color.clone().set_a(alpha)
    }

    fn get_radius(&self) -> f32 {
        self.radius * (0.5 + self.timer.percent())
    }

    pub fn done(&self) -> bool {
        self.timer.percent() >= 1.0
    }
}

#[derive(Copy, Clone)]
pub enum FlasherType {
    WeakShot,
    BigShot,
    HugeShot,
    HotShot,
    MiniShot,
    EnemyDeath,
    BumperDeath,
}

impl FlasherType {
    fn get_stats(&self) -> (f32, Color, f32) {
        match self {
            // Initial radius, Color, time to dissipate
            Self::WeakShot => (12.0, Color::YELLOW, 1.4),
            Self::BigShot => (20.0, Color::ORANGE, 1.8),
            Self::HugeShot => (24.0, Color::WHITE, 2.4),
            Self::HotShot => (15.0, Color::RED, 2.2),
            Self::MiniShot => (8.0, Color::BLUE, 1.2),
            Self::EnemyDeath => (18.0, Color::PINK, 2.2),
            Self::BumperDeath => (10.0, Color::GRAY, 4.0),
        }
    }
}
