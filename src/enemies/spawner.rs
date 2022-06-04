use bevy::core::Timer;
use bevy::ecs::component::Component;
use bevy::utils::Duration;

use crate::enemies::enemy::Enemy;

const MAX_ENEMIES: usize = 15;

#[derive(Component)]
pub struct Spawner {
    timer: Timer,
    enemy_count: usize,
    enemies_spawned: usize,
}

impl Spawner {
    pub fn new(secs: f32) -> Spawner {
        Spawner {
            timer: Timer::from_seconds(secs, true),
            enemy_count: 0,
            enemies_spawned: 0,
        }
    }

    fn spawn(&mut self) -> Enemy {
        self.enemy_count += 1;
        self.enemies_spawned += 1;
        Enemy::create_random()
    }

    pub fn spawn_if_ready(&mut self, duration: Duration) -> Option<Enemy> {
        if !self.timer.tick(duration).just_finished() {
            None
        } else if self.enemy_count >= MAX_ENEMIES {
            None
        } else {
            Some(self.spawn())
        }
    }

    pub fn despawn(&mut self) {
        self.enemy_count -= 1;
    }
}
