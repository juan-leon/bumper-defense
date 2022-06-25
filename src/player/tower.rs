use bevy::ecs::component::Component;
use bevy::math::{const_vec3, Vec2, Vec3};

use crate::util::lifebar::Lifebar;
use crate::world;
use crate::world::landscape;

pub const POSITION: f32 = 500.0;
pub const WIDTH: f32 = 15.0;
pub const HEIGHT: f32 = 35.0;

const INITIAL_LIFE: f32 = 10000.0;

#[derive(Component)]
pub struct Tower {
    life: f32,
    location: Vec3,
}

impl Tower {
    pub fn new() -> Tower {
        Tower {
            life: INITIAL_LIFE,
            location: const_vec3!([
                POSITION,
                HEIGHT + landscape::FLOOR_HEIGHT / 2.0,
                world::Layer::Player.to_z()
            ]),
        }
    }

    pub fn translation(&self) -> Vec3 {
        self.location
    }

    pub fn scale(&self) -> Vec3 {
        const_vec3!([WIDTH, HEIGHT, 0.0])
    }

    pub fn shape(&self) -> Vec3 {
        const_vec3!([WIDTH / 2.0, HEIGHT / 2.0, 0.0])
    }

    // FIXME: implememt trait "life" with these tree following funcs in default
    // impl
    pub fn take_damage(&mut self, damage: f32) {
        self.life -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }

    pub fn get_lifebar(&self) -> Option<Lifebar> {
        let relative_position = Vec3::new(-WIDTH / 2.0, (HEIGHT + 4.0) / 2.0, 0.0);
        if self.life > 0.0 {
            Some(Lifebar::new(
                self.location + relative_position,
                self.life / INITIAL_LIFE,
            ))
        } else {
            None
        }
    }

    // Returns a pseudo collision shape in the form of (top_left, bottom_right)
    // tuple
    pub fn as_rect(&self) -> (Vec2, Vec2) {
        (
            self.location.truncate() + Vec2::new(-WIDTH / 2.0, HEIGHT / 2.0),
            self.location.truncate() + Vec2::new(WIDTH / 2.0, -HEIGHT / 2.0),
        )
    }
}
