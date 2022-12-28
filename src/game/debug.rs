use super::{super::log, collision};
use bevy::prelude::*;

const DEBUG_DRAW_Z: f32 = 2.;

const AXIS_WIDTH: f32 = 5.;
const AXIS_HEIGHT: f32 = 2000.;

const AXIS_INCREMENTS: f32 = 50.;
const AXIS_INCREMENT_SIZE: f32 = 5.;

const NUM_INCREMENTS: i32 = (AXIS_HEIGHT / AXIS_INCREMENTS) as i32;

const X_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);
const Y_COLOR: Color = Color::rgb(0.25, 0.75, 0.25);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DrawToggleResource>()
            .add_startup_system(log_in_use)
            .add_startup_system(setup_increments)
            .add_startup_system(setup_text)
            .add_system(draw_collision_rectangles);
    }
}

#[derive(Resource, Default, Clone)]
pub struct DrawToggleResource {
    draw_collision_rectangles: bool,
    draw_collision_rectangles_combined: bool,
}

fn draw_collision_rectangles(
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
            for rectangle in a.combined_rectangles.iter() {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: X_COLOR,
                        custom_size: Some(Vec2::new(
                            rectangle.width as f32,
                            rectangle.height as f32,
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        rectangle.x as f32,
                        rectangle.y as f32,
                        DEBUG_DRAW_Z,
                    )),
                    ..default()
                });
            }
        } else {
        }
    }
}

fn log_in_use() {
    log::log!("Loading Debug Plugin!")
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    for count in 1..=NUM_INCREMENTS {
        let x_translation = AXIS_WIDTH * 2.0;
        let y_translation = count as f32 * AXIS_INCREMENTS;

        commands.spawn((Text2dBundle {
            text: Text::from_section(
                format!("{}", count as f32 * AXIS_INCREMENTS),
                text_style.clone(),
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform: Transform::from_translation(Vec3::new(
                x_translation,
                y_translation,
                DEBUG_DRAW_Z,
            )),
            ..default()
        },));

        commands.spawn((Text2dBundle {
            text: Text::from_section(
                format!("{}", count as f32 * AXIS_INCREMENTS),
                text_style.clone(),
            )
            .with_alignment(TextAlignment::BOTTOM_CENTER),
            transform: Transform::from_translation(Vec3::new(
                y_translation,
                x_translation,
                DEBUG_DRAW_Z,
            )),
            ..default()
        },));
    }
}

fn setup_increments(mut commands: Commands) {
    // X Axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: X_COLOR,
            custom_size: Some(Vec2::new(AXIS_HEIGHT, AXIS_WIDTH)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            AXIS_HEIGHT / 2.0,
            AXIS_WIDTH / 2.0,
            DEBUG_DRAW_Z,
        )),
        ..default()
    });

    // Y Axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Y_COLOR,
            custom_size: Some(Vec2::new(AXIS_WIDTH, AXIS_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            AXIS_WIDTH / 2.0,
            AXIS_HEIGHT / 2.0,
            DEBUG_DRAW_Z,
        )),
        ..default()
    });

    for count in 1..=NUM_INCREMENTS {
        let x_translation = (count as f32 * AXIS_INCREMENTS) - (AXIS_INCREMENT_SIZE / 2.0);
        let y_translation = (AXIS_WIDTH * 1.5) - (AXIS_INCREMENT_SIZE / 2.0);

        // X Axis
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: X_COLOR,
                custom_size: Some(Vec2::new(AXIS_INCREMENT_SIZE, AXIS_INCREMENT_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x_translation,
                y_translation,
                DEBUG_DRAW_Z,
            )),
            ..default()
        });

        // Y Axis
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Y_COLOR,
                custom_size: Some(Vec2::new(AXIS_INCREMENT_SIZE, AXIS_INCREMENT_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                y_translation,
                x_translation,
                DEBUG_DRAW_Z,
            )),
            ..default()
        });
    }
}
