use bevy::ecs::component::Component;
use bevy::math::{const_vec3, Vec3};
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;

const HEIGHT: f32 = 2.0;
const MAX_LENGHT: f32 = 15.0;
const Z_VEC: Vec3 = const_vec3!([-10.0, 12.0, 50.0]);

#[derive(Component)]
pub struct Lifebar {
    origin: Vec3,
    pctg: f32,
}

impl Lifebar {
    pub fn new(origin: Vec3, pctg: f32) -> Lifebar {
        Lifebar {
            origin: origin + Z_VEC,
            pctg: pctg,
        }
    }

    pub fn sprite(&self) -> SpriteBundle {
        let bar_len = self.lenght();
        SpriteBundle {
            transform: Transform {
                translation: self.origin + (Vec3::X * bar_len / 2.0),
                scale: Vec3::new(bar_len, HEIGHT, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: self.color(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn lenght(&self) -> f32 {
        (MAX_LENGHT * self.pctg).clamp(1.0, MAX_LENGHT)
    }

    fn color(&self) -> Color {
        match self.pctg {
            x if x < 0.2 => Color::RED,
            x if x < 0.5 => Color::ORANGE,
            _ => Color::GREEN,
        }
    }
}
