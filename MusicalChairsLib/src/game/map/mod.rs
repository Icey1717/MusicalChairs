#[cfg_attr(not(feature = "graphics"), path = "headless_map.rs")]
#[cfg_attr(feature = "graphics", path = "graphics_map.rs")]
pub mod map;

use crate::log;
use bevy::prelude::*;

use super::game::AppState;

pub fn goto_in_game(mut app_state: ResMut<State<AppState>>) {
    log::log!("Transitioning to AppState::InGame");
    app_state.set(AppState::InGame).unwrap();
}
