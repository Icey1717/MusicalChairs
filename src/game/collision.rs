use crate::game::player::player;

use super::super::log;
use super::game;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CollisionResource {
    pub rectangles: Vec<Rectangle>,
    pub precomputed_rectangles: Vec<PrecomputedRectangle>,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionResource>().add_system_set(
            SystemSet::on_enter(game::AppState::Loaded).with_system(build_collision),
        );
    }
}

#[derive(Component, Default)]
pub struct StaticCar;

fn build_collision(
    car_query: Query<(&StaticCar, &Transform)>,
    mut collision_resource: ResMut<CollisionResource>,
) {
    log::log!("Building collision.");
    log::log!("Query returned {:?}.", car_query);

    for (_car, transform) in car_query.iter() {
        log::log!("T: {:?}.", transform);
        let rect = Rectangle {
            x: transform.translation.x as i32,
            y: transform.translation.y as i32,
            width: player::CAR_SIZE_PX.x as i32,
            height: player::CAR_SIZE_PX.y as i32,
            rotation: 0.0,
        };
        collision_resource.rectangles.push(rect);
        collision_resource
            .precomputed_rectangles
            .push(PrecomputedRectangle::from_rect(&rect));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rotation: f64,
}

use nalgebra::{Matrix2, Vector2};

pub fn get_world_space_vertices(rect: &Rectangle) -> [Vector2<f64>; 4] {
    let half_width = rect.width as f64 / 2.0;
    let half_height = rect.height as f64 / 2.0;

    let rotation_matrix = Matrix2::new(
        rect.rotation.cos(),
        -rect.rotation.sin(),
        rect.rotation.sin(),
        rect.rotation.cos(),
    );

    // The vertices of the rectangle in local space
    let vertices = [
        Vector2::new(-half_width, -half_height),
        Vector2::new(half_width, -half_height),
        Vector2::new(half_width, half_height),
        Vector2::new(-half_width, half_height),
    ];

    // Transform the vertices from local space to world space
    let mut world_space_vertices = [Vector2::zeros(); 4];
    for (i, vertex) in vertices.iter().enumerate() {
        world_space_vertices[i] =
            rotation_matrix * vertex + Vector2::new(rect.x as f64, rect.y as f64);
    }

    world_space_vertices
}

pub fn get_normals(rect: &Rectangle) -> [Vector2<f64>; 4] {
    let rotation_matrix = Matrix2::new(
        rect.rotation.cos(),
        -rect.rotation.sin(),
        rect.rotation.sin(),
        rect.rotation.cos(),
    );

    // The normals of the rectangle in local space
    let normals = [
        Vector2::new(0.0, -1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 0.0),
    ];

    // Transform the normals from local space to world space
    let mut world_space_normals = [Vector2::zeros(); 4];
    for (i, normal) in normals.iter().enumerate() {
        world_space_normals[i] = rotation_matrix * normal;
    }

    world_space_normals
}

pub struct PrecomputedRectangle {
    world_space_vertices: [Vector2<f64>; 4],
    world_space_normals: [Vector2<f64>; 4],
}

impl PrecomputedRectangle {
    pub fn from_rect(rect: &Rectangle) -> PrecomputedRectangle {
        PrecomputedRectangle {
            world_space_vertices: get_world_space_vertices(rect),
            world_space_normals: get_normals(rect),
        }
    }
}

pub fn separating_axis_test(
    rect1: &PrecomputedRectangle,
    rect2: &PrecomputedRectangle,
) -> Option<Vector2<f64>> {
    // Check for separating axis along the normals of rect1
    for normal in &rect1.world_space_normals {
        let (min1, max1) = get_projection_range(&rect1.world_space_vertices, normal);
        let (min2, max2) = get_projection_range(&rect2.world_space_vertices, normal);
        if max1 < min2 || max2 < min1 {
            // The projections of the rectangles onto the normal do not overlap,
            // so the rectangles do not intersect
            return None;
        }
    }

    // Check for separating axis along the normals of rect2
    for normal in &rect2.world_space_normals {
        let (min1, max1) = get_projection_range(&rect1.world_space_vertices, normal);
        let (min2, max2) = get_projection_range(&rect2.world_space_vertices, normal);
        if max1 < min2 || max2 < min1 {
            // The projections of the rectangles onto the normal do not overlap,
            // so the rectangles do not intersect
            return None;
        }
    }

    // If no separating axis was found, the rectangles must intersect
    // Find the shortest distance between the rectangles along one of the intersecting edges
    let mut min_distance = f64::INFINITY;
    let mut normal = Vector2::zeros();
    for norm in &rect1.world_space_normals {
        let distance = get_distance_between_rectangles(rect1, rect2, norm);
        if distance < min_distance {
            min_distance = distance;
            normal = norm.clone();
        }
    }
    for norm in &rect2.world_space_normals {
        let distance = get_distance_between_rectangles(rect1, rect2, norm);
        if distance < min_distance {
            min_distance = distance;
            normal = norm.clone();
        }
    }
    Some(normal)
}

fn get_distance_between_rectangles(
    rect1: &PrecomputedRectangle,
    rect2: &PrecomputedRectangle,
    normal: &Vector2<f64>,
) -> f64 {
    let (min1, _max1) = get_projection_range(&rect1.world_space_normals, normal);
    let (_min2, max2) = get_projection_range(&rect2.world_space_normals, normal);
    (max2 - min1).abs()
}

fn get_projection_range(vertices: &[Vector2<f64>], normal: &Vector2<f64>) -> (f64, f64) {
    // Project each vertex of the rectangle onto the normal and return the
    // minimum and maximum projections
    let mut min_projection = f64::INFINITY;
    let mut max_projection = f64::NEG_INFINITY;
    for vertex in vertices {
        let projection = vertex.dot(normal);
        min_projection = min_projection.min(projection);
        max_projection = max_projection.max(projection);
    }
    (min_projection, max_projection)
}
