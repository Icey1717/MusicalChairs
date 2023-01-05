use bevy::prelude::*;

const AXIS_WIDTH: f32 = 5.;
const AXIS_HEIGHT: f32 = 2000.;

const AXIS_INCREMENTS: f32 = 50.;
const AXIS_INCREMENT_SIZE: f32 = 5.;

const NUM_INCREMENTS: i32 = (AXIS_HEIGHT / AXIS_INCREMENTS) as i32;

pub fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                super::DEBUG_DRAW_Z,
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
                super::DEBUG_DRAW_Z,
            )),
            ..default()
        },));
    }
}

pub fn setup_increments(mut commands: Commands) {
    // X Axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: super::X_COLOR,
            custom_size: Some(Vec2::new(AXIS_HEIGHT, AXIS_WIDTH)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            AXIS_HEIGHT / 2.0,
            AXIS_WIDTH / 2.0,
            super::DEBUG_DRAW_Z,
        )),
        ..default()
    });

    // Y Axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: super::Y_COLOR,
            custom_size: Some(Vec2::new(AXIS_WIDTH, AXIS_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            AXIS_WIDTH / 2.0,
            AXIS_HEIGHT / 2.0,
            super::DEBUG_DRAW_Z,
        )),
        ..default()
    });

    for count in 1..=NUM_INCREMENTS {
        let x_translation = (count as f32 * AXIS_INCREMENTS) - (AXIS_INCREMENT_SIZE / 2.0);
        let y_translation = (AXIS_WIDTH * 1.5) - (AXIS_INCREMENT_SIZE / 2.0);

        // X Axis
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: super::X_COLOR,
                custom_size: Some(Vec2::new(AXIS_INCREMENT_SIZE, AXIS_INCREMENT_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                x_translation,
                y_translation,
                super::DEBUG_DRAW_Z,
            )),
            ..default()
        });

        // Y Axis
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: super::Y_COLOR,
                custom_size: Some(Vec2::new(AXIS_INCREMENT_SIZE, AXIS_INCREMENT_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                y_translation,
                x_translation,
                super::DEBUG_DRAW_Z,
            )),
            ..default()
        });
    }
}
