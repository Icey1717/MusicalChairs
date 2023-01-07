pub mod player_car;

use bevy::prelude::*;

#[cfg(feature = "graphics")]
pub mod graphics;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerLoadState {
    Setup,
    GraphicsLoaded,
    Finished,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        check_graphics_setup(app);
        app.add_system_set(
            SystemSet::on_update(PlayerLoadState::GraphicsLoaded).with_system(player_car::setup),
        )
        .add_system_set(SystemSet::on_update(PlayerLoadState::Finished).with_system(player_car::tick));
    }
}

#[cfg(not(feature = "graphics"))]
fn check_graphics_setup(app: &mut App) {
    app.add_state(PlayerLoadState::GraphicsLoaded);
}

#[cfg(feature = "graphics")]
fn check_graphics_setup(app: &mut App) {
    app.add_plugin(graphics::PlayerGraphicsPlugin);
}
