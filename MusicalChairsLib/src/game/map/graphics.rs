use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game;

use super::super::super::log;
use super::super::collision;

pub fn add_systems(app: &mut App) -> &mut App {
    app.add_event::<MapDataLoadedEvent>()
        // systems to run only while loading
        .add_system_set(
            SystemSet::on_update(game::AppState::Loading).with_system(wait_for_level_spawned),
        )
        .add_system_set(
            SystemSet::on_enter(game::AppState::Loaded)
                .with_system(on_car_loaded)
                .with_system(on_level_loaded),
        )
        .add_system_set(
            SystemSet::on_update(game::AppState::Loaded)
                .with_system(camera_wait_for_map)
                .with_system(window_wait_for_map)
                .with_system(super::goto_in_game),
        )
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
        .register_ldtk_entity::<CarBundle>("Car")
        .add_startup_system(setup_map)
}

const NUM_CAR_SPRITES: usize = 5;

struct MapData {
    width: i32,
    height: i32,
}

struct MapDataLoadedEvent(MapData);

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels\\simple.ldtk"),
        ..Default::default()
    });
}

fn camera_2d_translation(x: f32, y: f32) -> Vec3 {
    return Vec3 {
        x: x,
        y: y,
        z: game::CAMERA_FAR,
    };
}

#[derive(Bundle, LdtkEntity)]
pub struct CarBundle {
    #[sprite_sheet_bundle("textures\\cars.png", 44.0, 74.0, 1, 5, 0.0, 0.0, 1)]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    car: collision::StaticCar,
}

fn wait_for_level_spawned(
    mut ev_level: EventReader<LevelEvent>,
    mut app_state: ResMut<State<game::AppState>>,
) {
    if ev_level.is_empty() {
        log::log!("Loading!");
        return;
    }

    for a in ev_level.iter() {
        log::log!("{:?}", a);

        match a {
            LevelEvent::SpawnTriggered(..) => log::log!("Received event spawn triggerred!"),
            LevelEvent::Spawned(..) => app_state.set(game::AppState::Loaded).unwrap(),
            _ => log::log!("Receive unhandled event!"),
        }
    }
}

#[cfg(feature = "serialize_collision")]
fn on_car_loaded(
    mut car_query: Query<(&collision::StaticCar, &mut TextureAtlasSprite, &Transform)>,
) {
    let mut transforms: Vec<Transform> = Vec::new();

    for (_car, mut sprite, transform) in car_query.iter_mut() {
        let x = rand::random::<usize>();
        sprite.index = x % NUM_CAR_SPRITES;
        transforms.push(*transform);
    }

    use std::fs::File;
    use std::io::BufWriter;

    log::log!("Serializing collision to file.");

    // Serialize the transforms to a BufWriter.
    let file = match File::create("transforms.bin") {
        Ok(file) => file,
        Err(error) => {
            log::log!("Error creating file! Error: {}", error);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    let encoded: Vec<u8> = bincode::serialize(&transforms).unwrap();
    log::log!("{:?}", encoded);

    let _result = match bincode::serialize_into(&mut writer, &transforms) {
        Ok(_result) => (),
        Err(error) => {
            log::log!("Error serializing file! Error: {}", error);
            return;
        }
    };
}

#[cfg(not(feature = "serialize_collision"))]
fn on_car_loaded(mut car_query: Query<(&collision::StaticCar, &mut TextureAtlasSprite)>) {
    for (_car, mut sprite) in car_query.iter_mut() {
        let x = rand::random::<usize>();
        sprite.index = x % NUM_CAR_SPRITES;
    }
}

fn on_level_loaded(
    level_query: Query<&Handle<LdtkLevel>>,
    level_assets: Res<Assets<LdtkLevel>>,
    mut ev_map_loaded: EventWriter<MapDataLoadedEvent>,
) {
    log::log!("{:?}", level_query);
    for level_handle in level_query.iter() {
        log::log!("{:?}", level_handle);
        if let Some(level) = level_assets.get(&level_handle) {
            ev_map_loaded.send(MapDataLoadedEvent(MapData {
                width: level.level.px_wid,
                height: level.level.px_hei,
            }));
        }
    }
}

fn camera_wait_for_map(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut ev_map_loaded: EventReader<MapDataLoadedEvent>,
) {
    for ev in ev_map_loaded.iter() {
        for (_camera, mut transform) in camera_query.iter_mut() {
            (*transform).translation =
                camera_2d_translation((ev.0.width / 2) as f32, (ev.0.height / 2) as f32);
        }
    }
}

fn window_wait_for_map(
    mut windows: ResMut<Windows>,
    mut ev_map_loaded: EventReader<MapDataLoadedEvent>,
) {
    for ev in ev_map_loaded.iter() {
        let window = windows.primary_mut();
        window.set_resolution(ev.0.width as f32, ev.0.height as f32)
    }
}
