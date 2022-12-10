use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::game;
use super::super::log;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app

        // systems to run only while loading
        .add_system_set(
            SystemSet::on_update(game::AppState::Loading)
                .with_system(on_camera_loaded)
                .with_system(on_car_loaded)
        )

        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            set_clear_color: SetClearColor::FromLevelBackground,
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
        .register_ldtk_entity::<CarBundle>("Car")
        .add_startup_system(setup_map);
    }
}

const MAP_DIMENSION_INVALID: i32 = -1;

struct MapData {
    width: i32,
    height: i32,
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels\\simple.ldtk"),
        ..Default::default()
    });
}

fn camera_2d_translation(x: f32, y: f32) -> Vec3 {
    return Vec3 { x: x, y: y, z: game::CAMERA_FAR };
}

#[derive(Component, Default)]
struct Car;


#[derive(Bundle, LdtkEntity)]
pub struct CarBundle {
    #[sprite_sheet_bundle("textures\\cars.png", 44.0, 74.0, 1, 5, 0.0, 0.0, 1)]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    car: Car,
}

fn wait_for_level_event<F: FnMut()>(ev_level: &mut EventReader<LevelEvent>, mut f: F)
{
    if ev_level.is_empty() {
        log::log!("Loading!");
        return;
    }
    
    for a in ev_level.iter() {
        log::log!("{:?}", a);

        match a {
            LevelEvent::SpawnTriggered(..) => log::log!("Received event spawn triggerred!"),
            LevelEvent::Spawned(..) => f(),
            _ => log::log!("Receive unhandled event!"),
        }
    }
}

fn on_car_loaded(mut car_query: Query<(&Car, &mut TextureAtlasSprite)>, mut ev_level: EventReader<LevelEvent>) {
    wait_for_level_event(&mut ev_level, || {
        for (_car, mut sprite) in car_query.iter_mut() {
            let x = rand::random::<u8>();
            sprite.index = x as usize % 5;
        }
    });
}

fn update_camera(camera_query: &mut Query<(&Camera, &mut Transform)>, level_query: &Query<&Handle<LdtkLevel>>, level_assets: &Res<Assets<LdtkLevel>>) {
    let mut map_data = MapData {width: MAP_DIMENSION_INVALID, height: MAP_DIMENSION_INVALID};
    log::log!("{:?}", level_query);
    for level_handle in level_query.iter() {
        log::log!("{:?}", level_handle);
        if let Some(level) = level_assets.get(&level_handle) {
            map_data.width = level.level.px_wid;
            map_data.height = level.level.px_hei;
        }
    }
    for (_camera, mut transform) in camera_query.iter_mut() {
        (*transform).translation = camera_2d_translation((map_data.width / 2) as f32, (map_data.height / 2) as f32);
    }
}

fn on_camera_loaded(mut camera_query: Query<(&Camera, &mut Transform)>, level_query: Query<&Handle<LdtkLevel>>, level_assets: Res<Assets<LdtkLevel>>, mut ev_level: EventReader<LevelEvent>, mut app_state: ResMut<State<game::AppState>>) {
    wait_for_level_event(&mut ev_level, || {
        update_camera(&mut camera_query, &level_query, &level_assets);
        app_state.set(game::AppState::InGame).unwrap();
    });
}