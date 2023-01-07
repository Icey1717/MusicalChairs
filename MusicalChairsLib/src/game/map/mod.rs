#[cfg_attr(not(feature = "graphics"), path = "headless.rs")]
#[cfg_attr(feature = "graphics", path = "graphics.rs")]
pub mod map;

#[cfg(not(feature = "graphics"))]
#[cfg(not(feature = "load_collision_from_file"))]
mod collision_data;

use crate::log;
use bevy::prelude::*;

use super::AppState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        map::add_systems(app);
    }
}

pub fn goto_in_game(mut app_state: ResMut<State<AppState>>) {
    log::log!("Transitioning to AppState::InGame");
    app_state.set(AppState::InGame).unwrap();
}
