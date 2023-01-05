use bevy::prelude::*;

use crate::game::collision;

#[derive(Resource, Default, Clone)]
pub struct DrawToggleResource {
    draw_collision_rectangles: bool,
}

pub fn draw_collision_rectangles(
    (a, mut b): (
        Res<collision::CollisionResource>,
        ResMut<DrawToggleResource>,
    ),
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let initial_values = b.clone();

    if keyboard_input.pressed(KeyCode::A) {
        b.draw_collision_rectangles = !b.draw_collision_rectangles;
    }

    if initial_values.draw_collision_rectangles != b.draw_collision_rectangles {
        if b.draw_collision_rectangles {
            for rectangle in a.rectangles.iter() {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: super::X_COLOR,
                        custom_size: Some(Vec2::new(
                            rectangle.width as f32,
                            rectangle.height as f32,
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        rectangle.x as f32,
                        rectangle.y as f32,
                        super::DEBUG_DRAW_Z,
                    )),
                    ..default()
                });
            }
        } else {
        }
    }
}
