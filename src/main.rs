use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod game;
mod log;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin) // Loads the ldtk map json file.
        .add_plugin(game::map::MapPlugin)
        .add_state(game::game::AppState::Loading)
        .add_startup_system(setup)
        
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle
    {
        transform: Transform::from_xyz(256.0, 256.0, game::game::CAMERA_FAR),
        ..Default::default()
    });
}