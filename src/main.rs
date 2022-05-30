mod enemies;
mod world;

use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

use heron::AxisAngle;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{const_vec3, Vec2, Vec3};

mod cli;

const GRAVITY: f32 = -600.;

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const SQUARE_SIZE: Vec3 = const_vec3!([20.0, 20.0, 0.0]);
const CUBOID_EXTENDS: Vec3 = const_vec3!([10.0, 10.0, 0.0]);
const NUM_BALLS: u32 = 15;
const NUM_BUMPERS: u32 = 8;

struct DropTimer(Timer);
#[derive(Component)]
struct Foo;

fn main() {
    let matches = cli::get_app().get_matches();
    let mut app = App::new();

    app.insert_resource(world::create_window(matches.value_of_t_or_exit("width")))
        .insert_resource(Gravity::from(Vec3::Y * GRAVITY))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(DropTimer(Timer::from_seconds(1.0, true)))
        .add_plugins(DefaultPlugins);
    if matches.is_present("verbose") {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
    app.add_plugin(PhysicsPlugin::default())
        .add_plugin(world::WorldPlugin)
        .add_plugin(enemies::EnemyPlugin)
        .add_startup_system(world::create_camera)
        //.add_startup_system(create_world)
        //.add_startup_system(create_balls)
        //.add_system(drop_stuff)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn create_balls(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(50.0),
        ..shapes::RegularPolygon::default()
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 6.0),
        },
        Transform {
            translation: Vec3::new(-600.0, 100.0, 0.0),
            ..default()
        },
    ));

    // https://docs.rs/bevy/latest/bevy/math/struct.Quat.html#method.from_axis_angle
    let shape2 = shapes::Line(Vec2::new(-300.0, -300.0), Vec2::new(300.0, 300.0));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape2,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 6.0),
        },
        Transform::default(),
    ));

    let shape3 = shapes::Rectangle {
        extents: Vec2::new(300.0, 6.0),
        origin: RectangleOrigin::Center,
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape3,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(1.0, 0.27, 0.0, 0.8)),
            outline_mode: StrokeMode::new(Color::rgba(1.0, 0.0, 0.0, 0.1), 1.5),
        },
        Transform {
            translation: Vec3::new(400.0, 100.0, 0.0),
            ..default()
        },
    ));

    let mut rng = thread_rng();
    for _i in 0..NUM_BALLS {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-300.0..300.0),
                        rng.gen_range(100.0..500.0),
                        0.0,
                    ),
                    scale: SQUARE_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: CUBOID_EXTENDS,
                border_radius: None,
            })
            .insert(
                Velocity::from_linear(Vec3::X * rng.gen_range(-90.0..90.0))
                    .with_angular(AxisAngle::new(Vec3::Z, rng.gen_range(0.0..0.8) * PI)),
            )
            //.insert(Acceleration::from_linear(Vec3::X * rng.gen_range(-20.0..20.0)))
            .insert(PhysicMaterial {
                friction: rng.gen_range(0.1..10.0),
                density: rng.gen_range(0.5..100.0),
                restitution: rng.gen_range(0.1..0.95),
            })
            .insert(RotationConstraints::allow())
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::Player)
                    .with_masks(&[world::Layer::World, world::Layer::Player]),
            );
    }

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-700.0, 500.0, 0.0),
                scale: SQUARE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(
                    rng.gen_range(0.6..1.0),
                    rng.gen_range(0.6..1.0),
                    rng.gen_range(0.6..1.0),
                ),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: CUBOID_EXTENDS,
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: rng.gen_range(0.1..10.0),
            density: rng.gen_range(0.5..100.0),
            restitution: rng.gen_range(0.1..0.95),
        })
        .insert(RotationConstraints::allow())
        .insert(
            CollisionLayers::none()
                .with_group(world::Layer::Player)
                .with_masks(&[world::Layer::World, world::Layer::Player]),
        )
        .insert(Foo);
    for _i in 0..NUM_BALLS {
        commands
            .spawn_bundle((
                Transform::from_translation(Vec3::new(
                    rng.gen_range(-300.0..300.0),
                    rng.gen_range(100.0..500.0),
                    0.0,
                )),
                GlobalTransform::default(),
            ))
            // .spawn_bundle(SpriteBundle {
            //     transform: Transform {
            //         translation: Vec3::new(rng.gen_range(-300.0..300.0), rng.gen_range(100.0..500.0), 0.0),
            //         scale: const_vec3!([0.045, 0.045, 0.1]),
            //         ..default()
            //     },
            //     texture: asset_server.load("red_ball.png"),
            //     ..default()
            // })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 11.0 })
            .insert(
                Velocity::from_linear(Vec3::X * rng.gen_range(-90.0..90.0))
                    .with_angular(AxisAngle::new(Vec3::Z, rng.gen_range(0.0..0.8) * PI)),
            )
            //.insert(Acceleration::from_linear(Vec3::X * rng.gen_range(-20.0..20.0)))
            .insert(PhysicMaterial {
                friction: rng.gen_range(0.1..10.0),
                density: rng.gen_range(0.5..100.0),
                restitution: rng.gen_range(0.1..0.95),
            })
            .insert(RotationConstraints::allow())
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::Player)
                    .with_masks(&[world::Layer::World, world::Layer::Player]),
            );
    }
}

fn create_world(mut commands: Commands) {
    let color = Color::rgb(0.2, 0.4, 0.8);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-0.0, -300., 0.0),
                scale: Vec3::new(700.0, 20.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: color,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: const_vec3!([350.0, 10.0, 0.0]),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 10.0,
            restitution: 1.0,
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(world::Layer::World)
                .with_mask(world::Layer::Player),
        );

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-440.0, -100., 0.0),
                scale: const_vec3!([20.0, 400.0, 0.0]),
                ..default()
            },
            sprite: Sprite {
                color: color,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: const_vec3!([10.0, 200.0, 0.0]),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 10.0,
            restitution: 1.0,
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(world::Layer::World)
                .with_mask(world::Layer::Player),
        );

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(440.0, -100., 0.0),
                scale: const_vec3!([20.0, 400.0, 0.0]),
                ..default()
            },
            sprite: Sprite {
                color: color,
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: const_vec3!([10.0, 200.0, 0.0]),
            border_radius: None,
        })
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 10.0,
            restitution: 1.0,
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(world::Layer::World)
                .with_mask(world::Layer::Player),
        );

    let mut rng = thread_rng();
    for _i in 0..NUM_BUMPERS {
        let x: f32 = rng.gen_range(-400.0..400.0);
        let y: f32 = rng.gen_range(-100.0..300.0);
        commands
            .spawn_bundle((
                Transform::from_translation(Vec3::new(x, y, 0.0)),
                GlobalTransform::default(),
            ))
            .insert(create_bumper_shape())
            .insert(RigidBody::Static)
            .insert(PhysicMaterial {
                friction: 1.0,
                density: 10.0,
                restitution: 1.5,
            })
            .insert(RotationConstraints::lock())
            .insert(
                CollisionLayers::none()
                    .with_group(world::Layer::World)
                    .with_mask(world::Layer::Player),
            );
    }
}

fn create_bumper_shape() -> heron::CollisionShape {
    let mut rng = thread_rng();
    let side = 2.0;
    let length: f32 = rng.gen_range(50.0..200.0);
    let angle: f32 = rng.gen_range(0.0..PI);

    let x = length * angle.cos();
    let y = length * angle.sin();

    CollisionShape::ConvexHull {
        points: vec![
            Vec3::new(x - (side * angle.sin()), y + (side * angle.cos()), 0.0),
            Vec3::new(x + (side * angle.sin()), y - (side * angle.cos()), 0.0),
            Vec3::new(-x + (side * angle.sin()), -y - (side * angle.cos()), 0.0),
            Vec3::new(-x - (side * angle.sin()), -y + (side * angle.cos()), 0.0),
        ],
        border_radius: None,
    }
}

fn drop_stuff(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut timer: ResMut<DropTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        if keyboard_input.pressed(KeyCode::Left) {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            rng.gen_range(-300.0..300.0),
                            rng.gen_range(100.0..500.0),
                            0.0,
                        ),
                        scale: SQUARE_SIZE,
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Cuboid {
                    half_extends: CUBOID_EXTENDS,
                    border_radius: None,
                })
                .insert(
                    Velocity::from_linear(Vec3::X * rng.gen_range(-50.0..50.0))
                        .with_angular(AxisAngle::new(Vec3::Z, rng.gen_range(0.0..0.8) * PI)),
                )
                .insert(PhysicMaterial {
                    friction: rng.gen_range(0.1..10.0),
                    density: rng.gen_range(0.5..100.0),
                    restitution: rng.gen_range(0.1..0.95),
                })
                .insert(RotationConstraints::allow())
                .insert(
                    CollisionLayers::none()
                        .with_group(world::Layer::Player)
                        .with_masks(&[world::Layer::World, world::Layer::Player]),
                );
        } else if keyboard_input.pressed(KeyCode::Right) {
            commands
                .spawn_bundle((
                    Transform::from_translation(Vec3::new(
                        rng.gen_range(-300.0..300.0),
                        rng.gen_range(100.0..500.0),
                        0.0,
                    )),
                    GlobalTransform::default(),
                ))
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Sphere { radius: 11.0 })
                .insert(
                    Velocity::from_linear(Vec3::X * rng.gen_range(-50.0..50.0))
                        .with_angular(AxisAngle::new(Vec3::Z, rng.gen_range(0.0..0.8) * PI)),
                )
                .insert(PhysicMaterial {
                    friction: rng.gen_range(0.1..10.0),
                    density: rng.gen_range(0.5..100.0),
                    restitution: rng.gen_range(0.1..0.95),
                })
                .insert(RotationConstraints::allow())
                .insert(
                    CollisionLayers::none()
                        .with_group(world::Layer::Player)
                        .with_masks(&[world::Layer::World, world::Layer::Player]),
                );
        } else if keyboard_input.pressed(KeyCode::Up) {
            let x: f32 = rng.gen_range(-400.0..400.0);
            let y: f32 = rng.gen_range(-100.0..300.0);
            commands
                .spawn_bundle((
                    Transform::from_translation(Vec3::new(x, y, 0.0)),
                    GlobalTransform::default(),
                ))
                .insert(create_bumper_shape())
                .insert(RigidBody::Static)
                .insert(PhysicMaterial {
                    friction: 1.0,
                    density: 10.0,
                    restitution: 1.5,
                })
                .insert(RotationConstraints::lock())
                .insert(
                    CollisionLayers::none()
                        .with_group(world::Layer::World)
                        .with_mask(world::Layer::Player),
                );
        }
    }
}
