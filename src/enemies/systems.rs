use bevy::asset::AssetServer;
use bevy::core::Time;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::{With, Without};
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::transform::components::Transform;
use heron::{CollisionEvent, Velocity};

use crate::enemies::enemy::Enemy;
use crate::enemies::enemy::ShootResult;
use crate::enemies::explosion::ActiveExplosions;
use crate::enemies::flasher::{Flasher, FlasherType};
use crate::enemies::projectile::{LandEffect, Projectile};
use crate::enemies::spawner::Spawner;
use crate::player::tower::Tower;
use crate::util::lifebar::Lifebar;
use crate::util::properties::{Shooter, JustLanded, Exploded, BumperActivated, Done};
use crate::world;

pub fn spawn_enemy(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<Spawner>) {
    if let Some(enemy) = spawner.spawn_if_ready(time.delta()) {
        commands
            .spawn_bundle(enemy.sprite())
            .insert_bundle(enemy.get_bundle())
            .insert(enemy);
    }
}

pub fn despawn_enemy(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    enemy_q: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in enemy_q.iter() {
        if enemy.done() {
            spawner.despawn();
            commands.entity(entity).despawn();
        }
    }
}

pub fn manage_enemy_movement(
    mut commands: Commands,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity, Entity)>,
) {
    for (mut enemy, transform, mut velocity, entity) in enemy_q.iter_mut() {
        match enemy.adjust_velocity(transform.translation, velocity.linear) {
            Some(velocity_l) => {
                velocity.linear = velocity_l;
            }
            None => {
                commands.entity(entity).remove::<Velocity>();
                commands.entity(entity).insert(Shooter);
            }
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut enemy_q: Query<(&mut Enemy, &Transform, Entity), With<Shooter>>,
) {
    for (mut enemy, transform, entity) in enemy_q.iter_mut() {
        if !enemy.ready(time.delta()) {
            continue;
        }
        match enemy.shoot(transform.translation) {
            ShootResult::Fire(projectile) => {
                commands
                    .spawn_bundle(projectile.sprite(asset_server.as_ref()))
                    .insert_bundle(projectile.get_bundle())
                    .insert(projectile);
            }
            ShootResult::GoAway(velocity) => {
                // FIXME: make it go away with velocity
                commands.entity(entity).insert(velocity);
            }
            ShootResult::Pass => (),
        }
    }
}

pub fn projectile_expired(
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: ResMut<ActiveExplosions>,
    mut proj_q: Query<(&mut Projectile, &Transform, Entity), Without<Exploded>>,
) {
    for (mut projectile, transform, entity) in proj_q.iter_mut() {
        if let Some(explosion) = projectile.tick(time.delta(), transform.translation) {
            let flasher = explosion.get_flasher();
            explosions.add(explosion);
            commands.spawn_bundle(flasher.get_bundle()).insert(flasher);
            commands.entity(entity).remove::<JustLanded>();
            commands.entity(entity).insert(Exploded);
        }
    }
}

pub fn landed_system(
    mut commands: Commands,
    mut proj_q: Query<(&mut Projectile, Entity), With<JustLanded>>,
) {
    for (mut projectile, entity) in proj_q.iter_mut() {
        commands.entity(entity).remove::<JustLanded>();
        match projectile.touch_ground() {
            LandEffect::Bounce => (),
            LandEffect::Explode => {
                commands.entity(entity).insert(Exploded);
            }
        }
    }
}

pub fn exploded_system(
    mut commands: Commands,
    mut explosions: ResMut<ActiveExplosions>,
    mut projectile_q: Query<(&Transform, &mut Projectile, Entity), With<Exploded>>,
) {
    explosions.clear();
    for (transform, mut projectile, entity) in projectile_q.iter_mut() {
        commands.entity(entity).remove::<JustLanded>();
        commands.entity(entity).remove::<Exploded>();
        if let Some(explosion) = projectile.explode(transform.translation) {
            let flasher = explosion.get_flasher();
            explosions.add(explosion);
            commands.spawn_bundle(flasher.get_bundle()).insert(flasher);
        }
        if projectile.done() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn damage_enemy_system(
    mut commands: Commands,
    explosions: Res<ActiveExplosions>,
    mut enemy_q: Query<(&Transform, &mut Enemy)>,
) {
    for (transform, mut enemy) in enemy_q.iter_mut() {
        let (center, radius) = enemy.as_circle();
        if let Some(damage) = explosions.damage_to_circle(center, radius) {
            enemy.take_damage(damage);
            if enemy.is_dead() {
                let flasher =
                    Flasher::new(transform.translation.truncate(), FlasherType::EnemyDeath);
                commands.spawn_bundle(flasher.get_bundle()).insert(flasher);
                // FIXME: add score
            }
        }
    }
}

pub fn damage_tower_system(
    mut commands: Commands,
    explosions: Res<ActiveExplosions>,
    mut tower_q: Query<(&mut Tower, Entity)>,
) {
    for (mut tower, entity) in tower_q.iter_mut() {
        let (top_left, bottom_right) = tower.as_rect();
        if let Some(damage) = explosions.damage_to_rect(top_left, bottom_right) {
            tower.take_damage(damage);
            if tower.is_dead() {
                let flasher = Flasher::new(tower.translation().truncate(), FlasherType::EnemyDeath);
                commands.spawn_bundle(flasher.get_bundle()).insert(flasher);
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn manage_flasher(
    mut commands: Commands,
    time: Res<Time>,
    mut flasher_q: Query<(&mut Flasher, Entity)>,
) {
    for (mut flasher, entity) in flasher_q.iter_mut() {
        flasher.tick(time.delta());
        if flasher.done() {
            commands.entity(entity).despawn();
        } else {
            commands.entity(entity).insert_bundle(flasher.get_bundle());
        }
    }
}

pub fn despawner(mut commands: Commands, entity_q: Query<Entity, With<Done>>) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn collision_dectector(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        if event.is_started() {
            match projectile_exploded(event) {
                Some(e) => {
                    commands.entity(e).insert(Exploded);
                    commands.entity(e).remove::<JustLanded>();
                    continue;
                }
                None => (),
            }
            match projectile_landed(event) {
                Some(e) => {
                    commands.entity(e).insert(JustLanded);
                    continue;
                }
                None => (),
            }
            match projectile_bumped(event) {
                Some(e) => {
                    commands.entity(e).insert(BumperActivated);
                    continue;
                }
                None => (),
            }
        }
    }
}

fn projectile_exploded(event: &CollisionEvent) -> Option<Entity> {
    let (data1, data2) = event.clone().data();
    let (layers1, layers2) = (data1.collision_layers(), data2.collision_layers());
    let mut entity: Option<Entity> = None;
    if layers1.contains_group(world::Layer::Projectiles)
        && layers2.contains_group(world::Layer::Trigger)
    {
        entity = Some(data1.rigid_body_entity());
    } else if layers2.contains_group(world::Layer::Projectiles)
        && layers1.contains_group(world::Layer::Trigger)
    {
        entity = Some(data2.rigid_body_entity());
    }
    entity
}

fn projectile_landed(event: &CollisionEvent) -> Option<Entity> {
    let (data1, data2) = event.clone().data();
    let (layers1, layers2) = (data1.collision_layers(), data2.collision_layers());
    let mut entity: Option<Entity> = None;
    if layers1.contains_group(world::Layer::Projectiles)
        && layers2.contains_group(world::Layer::World)
    {
        entity = Some(data1.rigid_body_entity());
    } else if layers2.contains_group(world::Layer::Projectiles)
        && layers1.contains_group(world::Layer::World)
    {
        entity = Some(data2.rigid_body_entity());
    }
    entity
}

fn projectile_bumped(event: &CollisionEvent) -> Option<Entity> {
    let (data1, data2) = event.clone().data();
    let (layers1, layers2) = (data1.collision_layers(), data2.collision_layers());
    let mut entity: Option<Entity> = None;
    if layers1.contains_group(world::Layer::Projectiles)
        && layers2.contains_group(world::Layer::Bumpers)
    {
        entity = Some(data2.rigid_body_entity());
    } else if layers2.contains_group(world::Layer::Projectiles)
        && layers1.contains_group(world::Layer::Bumpers)
    {
        entity = Some(data1.rigid_body_entity());
    }
    entity
}

pub fn despawn_lifebar(mut commands: Commands, lifebar_q: Query<Entity, With<Lifebar>>) {
    for entity in lifebar_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_lifebar(mut commands: Commands, enemy_q: Query<&Enemy>) {
    for enemy in enemy_q.iter() {
        if let Some(lifebar) = enemy.get_lifebar() {
            commands.spawn_bundle(lifebar.sprite()).insert(lifebar);
        }
    }
}

pub fn update_tower_lifebar(mut commands: Commands, tower_q: Query<&Tower>) {
    for tower in tower_q.iter() {
        if let Some(lifebar) = tower.get_lifebar() {
            commands.spawn_bundle(lifebar.sprite()).insert(lifebar);
        }
    }
}
