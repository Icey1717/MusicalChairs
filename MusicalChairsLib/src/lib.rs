use bevy::prelude::*;

#[cfg(feature = "graphics")]
use bevy_ecs_ldtk::prelude::*;

#[cfg(feature = "python")]
use python::Config;

mod ai;
mod game;
mod log;

#[cfg(feature = "python")]
mod python;

#[cfg(not(feature = "graphics"))]
fn add_graphics_plugins(app: &mut App) -> &mut App {
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
    })
}

#[cfg(feature = "graphics")]
fn add_graphics_plugins(app: &mut App) -> &mut App {
    app.add_plugins(DefaultPlugins)
        .add_plugin(game::debug::DebugPlugin)
        .add_plugin(LdtkPlugin) // Loads the ldtk map json file.
}

pub fn add_base_plugins(app: &mut App) -> &mut App {
        add_graphics_plugins(app)
        .add_plugin(game::map::map::MapPlugin)
        .add_plugin(game::player::player::PlayerPlugin)
        .add_plugin(game::collision::CollisionPlugin)
        .add_plugin(ai::AiPlugin)
        .add_state(game::game::AppState::Loading)
        .add_startup_system(setup)
}

use entity_gym_rs::agent;

const AGENT_PATH: Option<String> = None;

pub fn run() {
    add_base_plugins(&mut App::new())
    .insert_non_send_resource(match AGENT_PATH {
        Some(path) => ai::AiPlayer(agent::load(path)),
        None => ai::AiPlayer(agent::random()),
    })
        .run();
}

#[cfg(feature = "python")]
pub fn run_headless(_: Config, agent: entity_gym_rs::agent::TrainAgent, _seed: u64) {
    add_base_plugins(&mut App::new())
    .insert_non_send_resource(ai::AiPlayer(Box::new(agent)))
    .run();
}

fn setup(mut commands: Commands) {
    log::log!("Welcome to Musical Cars. Running main setup!");
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(256.0, 256.0, game::game::CAMERA_FAR),
        ..Default::default()
    });
}
