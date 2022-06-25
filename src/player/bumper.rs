use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;


#[derive(Component)]
pub struct Bumper {
    life: f32,
    initial_life: f32,
    length: f32,
    angle: f32,
    bounciness: f32,
    color: Color,
}

impl Bumper {
    pub fn new() -> Bumper {
        Bumper {
            life: 20.0,
            initial_life: 20.0,
            length: 80.0,
            angle: PI / 5.0,
            bounciness: 0.95,
            color: Color::PINK,
        }
    }

    pub fn fixed_bundle(&self, transform: &Transform) -> ShapeBundle {
        let angle = dbg!(dbg!(transform.rotation).to_euler(EulerRot::XYZ)).2;


        let x = 50.0 * dbg!(angle).cos();
        let y = 50.0 * angle.sin();

        GeometryBuilder::build_as(
            &shapes::Line(
                Vec2::new(transform.translation.x - x, transform.translation.y - y),
                Vec2::new(transform.translation.x + x, transform.translation.y + y)
            ),
            DrawMode::Stroke(StrokeMode::new(Color::BLUE, 2.0)),
            Transform::default(),
        )


        // GeometryBuilder::build_as(
        //     &shapes::Line(Vec2::new(-50.0, 0.0), Vec2::new(50.0, 0.0)),
        //     DrawMode::Stroke(StrokeMode::new(Color::BLUE, 3.0)),
        //     *transform,
        // )
    }

    pub fn floating_bundle(&self) -> ShapeBundle {
        GeometryBuilder::build_as(
            &shapes::Line(Vec2::new(-50.0, 0.0), Vec2::new(50.0, 0.0)),
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, 2.0)),
            Transform::default(),
        )
    }
}
