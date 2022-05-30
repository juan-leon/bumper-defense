use bevy::core::{Time, Timer};
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, Res, ResMut};

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

    pub fn spawn(&mut self, mut commands: Commands) {
        if self.enemy_count >= MAX_ENEMIES {
            return;
        }
        self.enemy_count += 1;
        self.enemies_spawned += 1;
        Enemy::spawn(self.enemies_spawned, commands);
    }

    pub fn despawn(&mut self) {
        self.enemy_count -= 1;
    }
}

pub fn spawn_enemy(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<Spawner>) {
    if spawner.timer.tick(time.delta()).just_finished() {
        spawner.spawn(commands);
    }
}
