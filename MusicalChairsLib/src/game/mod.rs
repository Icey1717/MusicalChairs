pub mod collision;
pub mod debug;
pub mod map;
pub mod player;

use bevy::prelude::*;

pub const CAMERA_FAR: f32 = 1000.0 - 1.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Loaded,
    InGame,
    //Paused,
}

pub struct GameOverEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_state(AppState::Loading);
    }
}