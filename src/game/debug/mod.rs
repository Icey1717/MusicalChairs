mod car_debug;
mod collision_debug;
mod map_debug;

use super::{super::log, car::PlayerLoadState, game::AppState};
use bevy::prelude::*;

const DEBUG_DRAW_Z: f32 = 2.;

const X_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);
const Y_COLOR: Color = Color::rgb(0.25, 0.75, 0.25);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<collision_debug::DrawToggleResource>()
            .init_resource::<car_debug::PlayerDebugResource>()
            .add_startup_system(log_in_use)
            .add_startup_system(map_debug::setup_increments)
            .add_startup_system(map_debug::setup_text)
            .add_system(collision_debug::draw_collision_rectangles)
            .add_system_set(
                SystemSet::on_enter(AppState::Loaded).with_system(car_debug::setup_debug),
            )
            .add_system_set(
                SystemSet::on_update(PlayerLoadState::Finished)
                    .with_system(car_debug::update_debug),
            )
            .add_system_set(
                SystemSet::on_update(PlayerLoadState::Finished)
                    .with_system(car_debug::update_debug_sprites),
            );
    }
}

fn log_in_use() {
    log::log!("Loading Debug Plugin!")
}
