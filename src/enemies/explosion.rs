use bevy::math::{Vec2, Vec3};

use crate::enemies::flasher::{Flasher, FlasherType};

pub struct Explosion {
    location: Vec2,
    damage: f32,
    radius: f32,
    flasher_type: FlasherType,
}

impl Explosion {
    pub fn new(location: Vec3, t: ExplosionType) -> Explosion {
        let stats = t.get_stats();
        Explosion {
            location: location.truncate(),
            radius: stats.0,
            damage: stats.1,
            flasher_type: stats.2,
        }
    }

    pub fn get_flasher(&self) -> Flasher {
        Flasher::new(self.location, self.flasher_type)
    }
}

pub struct ActiveExplosions {
    list: Vec<Explosion>,
}

impl ActiveExplosions {
    pub fn new() -> ActiveExplosions {
        ActiveExplosions { list: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, e: Explosion) {
        self.list.push(e);
    }

    pub fn damage_to_circle(&self, center: Vec2, radius: f32) -> Option<f32> {
        let mut damage: f32 = 0.0;
        for e in self.list.iter() {
            if center.distance(e.location) < e.radius + radius {
                damage += e.damage;
            }
        }
        if damage > 0.0 {
            Some(damage)
        } else {
            None
        }
    }

    pub fn damage_to_rect(&self, top_left: Vec2, bottom_right: Vec2) -> Option<f32> {
        let mut damage: f32 = 0.0;
        for e in self.list.iter() {
            if (e.location.x + e.radius > top_left.x || e.location.x - e.radius < bottom_right.x)
                && (e.location.y + e.radius > bottom_right.y
                    || e.location.y - e.radius < top_left.y)
            {
                damage += e.damage;
            }
        }
        if damage > 0.0 {
            Some(damage)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum ExplosionType {
    WeakShot,
    BigShot,
    HugeShot,
    HotShot,
    MiniShot,
}

impl ExplosionType {
    fn get_stats(&self) -> (f32, f32, FlasherType) {
        match self {
            // Damage radius, Damage, FlasherType
            Self::WeakShot => (12.0, 10.0, FlasherType::WeakShot),
            Self::BigShot => (20.0, 15.0, FlasherType::BigShot),
            Self::HugeShot => (24.0, 20.0, FlasherType::HugeShot),
            Self::HotShot => (15.0, 40.0, FlasherType::HotShot),
            Self::MiniShot => (7.0, 8.0, FlasherType::MiniShot),
        }
    }
}
