use bevy::prelude::*;

use crate::{game::player::player_car::PlayerCar, log};

#[derive(Resource, Default)]
pub struct PlayerDebugResource {
    style: TextStyle,
}

#[derive(Component)]
pub struct PlayerDebug {
    id: Entity,
}

#[derive(Component)]
pub struct PlayerDebugWheel {
    id: Entity,
    back_wheel: bool,
}

#[derive(Bundle)]
struct PlayerDebugBundle {
    text: Text2dBundle,
    debug: PlayerDebug,
}

#[derive(Bundle)]
struct PlayerDebugSpriteBundle {
    #[bundle]
    sprite: SpriteBundle,
    debug: PlayerDebugWheel,
}

pub fn setup_debug(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug: ResMut<PlayerDebugResource>,
    player_query: Query<Entity, &PlayerCar>,
) {
    log::log!("Setting up car debug!");

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    debug.style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    log::log!("Car debug query: {:?}", player_query);

    for player in player_query.iter() {
        log::log!("Setting up debug for player: {:?}", player);
        commands.spawn(PlayerDebugBundle {
            text: Text2dBundle { ..default() },
            debug: PlayerDebug { id: player },
        });

        commands.spawn(PlayerDebugSpriteBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            debug: PlayerDebugWheel {
                id: player,
                back_wheel: true,
            },
        });

        commands.spawn(PlayerDebugSpriteBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            debug: PlayerDebugWheel {
                id: player,
                back_wheel: false,
            },
        });
    }
}

pub fn update_debug(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_text_query: Query<(&PlayerDebug, &mut Text, &mut Transform, Without<PlayerCar>)>,
    debug: Res<PlayerDebugResource>,
) {
    for (player_debug, mut text, mut text_transform, ()) in player_text_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            // do something with the components
            *text = Text::from_sections([TextSection::new(
                format!(
                    "v: x: {:.1}, y: {:.1}\np: x: {:.1}, y: {:.1}",
                    car.velocity.x,
                    car.velocity.y,
                    car_transform.translation.x,
                    car_transform.translation.y
                ),
                debug.style.clone(),
            )])
            .with_alignment(TextAlignment::CENTER);
            text_transform.translation = car_transform.translation;
            text_transform.translation.z += 1.0;
        }
    }
}

pub fn update_debug_sprites(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_sprite_query: Query<(&PlayerDebugWheel, &mut Transform, Without<PlayerCar>)>,
) {
    for (player_debug, mut sprite_transform, ()) in player_sprite_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            if player_debug.back_wheel {
                sprite_transform.translation.x = car.back_wheel.x;
                sprite_transform.translation.y = car.back_wheel.y;
            } else {
                sprite_transform.translation.x = car.front_wheel.x;
                sprite_transform.translation.y = car.front_wheel.y;
            }

            sprite_transform.translation.z = car_transform.translation.z + 1.0;
        }
    }
}
