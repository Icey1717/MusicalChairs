use bevy::prelude::*;

#[cfg(feature = "graphics")]
use bevy_ecs_ldtk::prelude::*;

mod game;
mod log;

#[cfg(not(feature = "graphics"))]
pub const LOAD_LDTK_MAP: bool = false;

#[cfg(feature = "graphics")]
pub const LOAD_LDTK_MAP: bool = true;

#[cfg(not(feature = "graphics"))]
fn add_graphics_plugins(app: &mut App) {
    use bevy::app::ScheduleRunnerSettings;
    use std::time::Duration;

    use bevy::input::InputPlugin;

    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        0.0,
    )))
    .add_plugins(MinimalPlugins)
    .add_plugin(AssetPlugin {
        watch_for_changes: false,
        ..Default::default()
    })
    .add_plugin(InputPlugin {
        ..Default::default()
    });
}

#[cfg(feature = "graphics")]
fn add_graphics_plugins(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_plugin(game::debug::DebugPlugin)
        .add_plugin(LdtkPlugin); // Loads the ldtk map json file.
}
fn main() {
    let mut app = App::new();
    add_graphics_plugins(&mut app);
    app.add_plugin(game::map::map::MapPlugin)
        .add_plugin(game::player::player::PlayerPlugin)
        .add_plugin(game::collision::CollisionPlugin)
        .add_state(game::game::AppState::Loading)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    log::log!("Welcome to Musical Cars. Running main setup!");
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(256.0, 256.0, game::game::CAMERA_FAR),
        ..Default::default()
    });
}
