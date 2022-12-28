use super::super::log;
use super::{car, game};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CollisionResource {
    pub rectangles: Vec<Rectangle>,
    pub combined_rectangles: Vec<Rectangle>,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionResource>().add_system_set(
            SystemSet::on_enter(game::AppState::Loaded).with_system(build_collision),
        );
    }
}

fn build_collision(
    car_query: Query<(&car::Car, &Transform)>,
    mut collision_resource: ResMut<CollisionResource>,
) {
    log::log!("Building collision.");
    log::log!("Query returned {:?}.", car_query);

    for (_car, sprite) in car_query.iter() {
        log::log!("T: {:?}.", sprite);
        collision_resource.rectangles.push(Rectangle {
            x: sprite.translation.x as i32,
            y: sprite.translation.y as i32,
            width: car::CAR_SIZE_PX.x as i32,
            height: car::CAR_SIZE_PX.y as i32,
        });
    }

    collision_resource.combined_rectangles =
        combine_rectangles(collision_resource.rectangles.as_slice());
    log::log!("T: {:?}.", collision_resource.combined_rectangles);
}

use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn to_rect(&self) -> Rect {
        Rect::new(
            (self.x - (self.width / 2)) as f32,
            (self.y - (self.height / 2)) as f32,
            (self.x + self.width / 2) as f32,
            (self.y + self.height / 2) as f32,
        )
    }
}

fn combine_rectangles(rectangles: &[Rectangle]) -> Vec<Rectangle> {
    let mut rectangles = rectangles.to_vec();
    rectangles.sort_by(|a, b| a.x.cmp(&b.x));

    let mut combined_rectangles: HashSet<Rectangle> = HashSet::new();
    for rectangle in rectangles.iter() {
        let mut combined = false;
        let mut to_remove: Option<Rectangle> = None;
        for combined_rectangle in combined_rectangles.iter() {
            if rectangle.x >= combined_rectangle.x
                && rectangle.x + rectangle.width <= combined_rectangle.x + combined_rectangle.width
                && rectangle.y >= combined_rectangle.y
                && rectangle.y + rectangle.height
                    <= combined_rectangle.y + combined_rectangle.height
            {
                combined = true;
                break;
            } else if rectangle.x <= combined_rectangle.x
                && rectangle.x + rectangle.width >= combined_rectangle.x + combined_rectangle.width
                && rectangle.y <= combined_rectangle.y
                && rectangle.y + rectangle.height
                    >= combined_rectangle.y + combined_rectangle.height
            {
                to_remove = Some(*combined_rectangle);
                let x = std::cmp::min(rectangle.x, combined_rectangle.x);
                let y = std::cmp::min(rectangle.y, combined_rectangle.y);
                let width = std::cmp::max(
                    rectangle.x + rectangle.width,
                    combined_rectangle.x + combined_rectangle.width,
                ) - x;
                let height = std::cmp::max(
                    rectangle.y + rectangle.height,
                    combined_rectangle.y + combined_rectangle.height,
                ) - y;
                combined_rectangles.insert(Rectangle {
                    x,
                    y,
                    width,
                    height,
                });
                combined = true;
                break;
            }
        }

        if !to_remove.is_none() {
            combined_rectangles.remove(&to_remove.unwrap());
        }

        if !combined {
            combined_rectangles.insert(*rectangle);
        }
    }
    combined_rectangles.into_iter().collect()
}
