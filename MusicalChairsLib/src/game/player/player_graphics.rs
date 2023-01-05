use bevy::{asset::LoadState, prelude::*};

use crate::log;

use super::player;

pub struct PlayerGraphicsPlugin;

impl Plugin for PlayerGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarSpriteHandles>()
            .add_state(player::PlayerLoadState::Setup)
            .add_system_set(
                SystemSet::on_enter(player::PlayerLoadState::Setup).with_system(load_textures),
            )
            .add_system_set(
                SystemSet::on_update(player::PlayerLoadState::Setup).with_system(check_textures),
            )
            .add_system_set(
                SystemSet::on_enter(player::PlayerLoadState::Finished).with_system(setup),
            );
    }
}

#[derive(Resource, Default)]
struct CarSpriteHandles {
    handle: Handle<Image>,
}

fn load_textures(mut car_sprite_handles: ResMut<CarSpriteHandles>, asset_server: Res<AssetServer>) {
    log::log!("Beginning PlayerLoadState::Setup. Loading textures.");
    car_sprite_handles.handle = asset_server.load("textures\\cars.png");
}

fn check_textures(
    mut state: ResMut<State<player::PlayerLoadState>>,
    car_sprite_handles: ResMut<CarSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_load_state(car_sprite_handles.handle.clone()) {
        state.set(player::PlayerLoadState::GraphicsLoaded).unwrap();
    }
}

fn setup(
    mut player_query: Query<(&player::PlayerCar, &mut Handle<TextureAtlas>)>,
    car_sprite_handles: Res<CarSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    log::log!("Beginning PlayerLoadState::Finished. Assigning texture atlases");

    let texture_atlas = TextureAtlas::from_grid(
        car_sprite_handles.handle.clone(),
        Vec2::new(44.0, 74.0),
        1,
        5,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    log::log!("{:?}", player_query);

    for (_car, mut handle) in player_query.iter_mut() {
        *handle = texture_atlas_handle.clone();
    }
}
