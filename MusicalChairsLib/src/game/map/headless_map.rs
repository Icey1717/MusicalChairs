use bevy::prelude::*;

use super::super::collision;
use crate::{
    game::game::{self, AppState},
    log,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(game::AppState::Loading).with_system(spawn_colliders),
        )
        .add_system_set(
            SystemSet::on_update(game::AppState::Loaded).with_system(super::goto_in_game),
        );
    }
}

#[derive(Bundle, Default)]
struct CarBundle {
    transform: Transform,
    car: collision::StaticCar,
}

#[cfg(feature = "load_collision_from_file")]
fn spawn_colliders(mut app_state: ResMut<State<AppState>>, mut commands: Commands) {
    use std::fs::File;
    use std::io::BufReader;

    log::log!("Deserializing collision from file.");

    let file = match File::open("transforms.bin") {
        Ok(file) => file,
        Err(error) => {
            log::log!("Error opening file! Error: {}", error);
            return;
        }
    };

    let mut reader = BufReader::new(file);

    let transforms: Vec<Transform> = match bincode::deserialize_from(&mut reader) {
        Ok(transforms) => transforms,
        Err(error) => {
            log::log!("Error deserializing file! Error: {}", error);
            return;
        }
    };

    for transform in transforms.iter() {
        commands.spawn(CarBundle {
            transform: *transform,
            ..default()
        });
    }

    log::log!("Transitioning to AppState::Loaded");
    app_state.set(AppState::Loaded).unwrap();
}

#[cfg(not(feature = "load_collision_from_file"))]
fn spawn_colliders(mut app_state: ResMut<State<AppState>>, mut commands: Commands) {
    log::log!("Deserializing collision from byte array.");

    let transforms: Vec<Transform> =
        bincode::deserialize(&super::inbuilt_collision::MAP_COLLISION_DATA[..]).unwrap();

    for transform in transforms.iter() {
        commands.spawn(CarBundle {
            transform: *transform,
            ..default()
        });
    }

    log::log!("Transitioning to AppState::Loaded");
    app_state.set(AppState::Loaded).unwrap();
}
