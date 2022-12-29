use bevy::math::vec2;
use bevy::{asset::LoadState, math::Vec3Swizzles, prelude::*};

use super::game;
use super::{super::log, collision};

pub struct PlayerPlugin;

pub const CAR_SIZE_PX: Vec2 = Vec2 { x: 44.0, y: 74.0 };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerLoadState {
    Setup,
    Finished,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarSpriteHandles>()
            .init_resource::<PlayerDebugResource>()
            .add_state(PlayerLoadState::Setup)
            .add_system_set(SystemSet::on_enter(PlayerLoadState::Setup).with_system(load_textures))
            .add_system_set(
                SystemSet::on_update(PlayerLoadState::Setup).with_system(check_textures),
            )
            .add_system_set(SystemSet::on_enter(PlayerLoadState::Finished).with_system(setup))
            .add_system_set(SystemSet::on_enter(game::AppState::Loaded).with_system(setup_debug))
            .add_system_set(SystemSet::on_update(PlayerLoadState::Finished).with_system(tick))
            .add_system_set(
                SystemSet::on_update(PlayerLoadState::Finished).with_system(update_debug),
            )
            .add_system_set(
                SystemSet::on_update(PlayerLoadState::Finished).with_system(update_debug_alt),
            );
    }
}

#[derive(Resource, Default)]
struct CarSpriteHandles {
    handle: Handle<Image>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<CarSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handle = asset_server.load("textures\\cars.png");
}

fn check_textures(
    mut state: ResMut<State<PlayerLoadState>>,
    rpg_sprite_handles: ResMut<CarSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_load_state(rpg_sprite_handles.handle.clone()) {
        state.set(PlayerLoadState::Finished).unwrap();
    }
}

#[derive(Resource, Default)]
struct PlayerDebugResource {
    style: TextStyle,
    player_entities: Vec<Entity>,
}

fn setup_debug(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug: ResMut<PlayerDebugResource>,
) {
    log::log!("Setting up car debug!");

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    debug.style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    for player in debug.player_entities.iter() {
        log::log!("Setting up debug for player: {:?}", player);
        commands.spawn(PlayerDebugBundle {
            text: Text2dBundle { ..default() },
            debug: PlayerDebug { id: *player },
        });

        commands.spawn(PlayerDebugSpriteBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            debug: PlayerDebug { id: *player },
        });

        commands.spawn(PlayerDebugSpriteBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            debug: PlayerDebug { id: *player },
        });
    }
}

fn setup(
    mut commands: Commands,
    rpg_sprite_handles: Res<CarSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut debug: ResMut<PlayerDebugResource>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        rpg_sprite_handles.handle.clone(),
        Vec2::new(44.0, 74.0),
        1,
        5,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // draw a sprite from the atlas
    debug.player_entities.push(
        commands
            .spawn(PlayerCarBundle {
                sprite: SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(450.0, 250.0, 1.0),
                        scale: Vec3::splat(1.0),
                        ..default()
                    },
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: texture_atlas_handle,
                    ..default()
                },
                player_car: PlayerCar {
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    max_speed: 100.0,
                    size: CAR_SIZE_PX,
                    heading: Vec2 { x: 1.0, y: 0.0 },
                    ..default()
                },
                ..default()
            })
            .id(),
    );
}

#[derive(Component, Default)]
pub struct Car;

#[derive(Component)]
struct PlayerDebug {
    id: Entity,
}

#[derive(Component, Default)]
struct PlayerCar {
    velocity: Vec2,
    acceleration: Vec2,
    max_speed: f32,
    size: Vec2,
    front_wheel: Vec2,
    back_wheel: Vec2,
    heading: Vec2,
}

#[derive(Component, Default)]
struct PlayerInput {
    throttle: f32,
    steering: f32,
}

fn rotate_vector(vector: Vec2, angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    Vec2::new(
        cos * vector.x - sin * vector.y,
        sin * vector.x + cos * vector.y,
    )
}

const WHEEL_BASE: f32 = 50.0;
const HALF_WHEEL_BASE: f32 = WHEEL_BASE / 2.0;

const MAX_STEERING: f32 = std::f32::consts::PI / 2.0;

impl PlayerCar {
    fn update_steering(&mut self, input: PlayerInput, delta_time: f32, position_2d: Vec2) {
        // Work out where the front and back wheels will be.
        self.back_wheel = position_2d - (self.heading * HALF_WHEEL_BASE);
        self.front_wheel = position_2d + (self.heading * HALF_WHEEL_BASE);

        self.back_wheel += self.velocity * delta_time;

        if input.steering.abs() > 0.0 {
            let steer_angle = MAX_STEERING * input.steering;
            self.front_wheel += rotate_vector(self.velocity, steer_angle) * delta_time;
        } else {
            self.front_wheel += self.velocity * delta_time;
        }

        // Update the forward and velocity.
        self.heading = (self.front_wheel - self.back_wheel).normalize();
    }

    fn get_rotatation_rads(&self) -> f32 {
        Vec2::new(1.0, 0.0).angle_between(self.heading)
    }
}

#[derive(Bundle)]
struct PlayerDebugBundle {
    text: Text2dBundle,
    debug: PlayerDebug,
}

#[derive(Bundle)]
struct PlayerDebugSpriteBundle {
    #[bundle]
    sprite: SpriteBundle,
    debug: PlayerDebug,
}

#[derive(Bundle, Default)]
pub struct PlayerCarBundle {
    player_car: PlayerCar,
    #[bundle]
    sprite: SpriteSheetBundle,
    input: PlayerInput,
}

fn update_debug(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_text_query: Query<(&PlayerDebug, &mut Text, &mut Transform, Without<PlayerCar>)>,
    debug: Res<PlayerDebugResource>,
) {
    for (player_debug, mut text, mut text_transform, ()) in player_text_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            // do something with the components
            *text = Text::from_sections([TextSection::new(
                format!(
                    "v: x: {:.1}, y: {:.1}\na: x: {:.1}, y: {:.1}\np: x: {:.1}, y: {:.1}",
                    car.velocity.x,
                    car.velocity.y,
                    car.acceleration.x,
                    car.acceleration.y,
                    car_transform.translation.x,
                    car_transform.translation.y
                ),
                debug.style.clone(),
            )])
            .with_alignment(TextAlignment::CENTER);
            text_transform.translation = car_transform.translation;
            text_transform.translation.z += 1.0;
        }
    }
}

fn update_debug_alt(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_sprite_query: Query<(
        &PlayerDebug,
        &mut Sprite,
        &mut Transform,
        Without<PlayerCar>,
    )>,
    debug: Res<PlayerDebugResource>,
) {
    let mut must_be_front = false;
    for (player_debug, mut text, mut text_transform, ()) in player_sprite_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            if must_be_front {
                text_transform.translation.x = car.front_wheel.x;
                text_transform.translation.y = car.front_wheel.y;
            } else {
                text_transform.translation.x = car.back_wheel.x;
                text_transform.translation.y = car.back_wheel.y;
            }

            text_transform.translation.z = car_transform.translation.z + 1.0;
            must_be_front = true;
        }
    }
}

fn get_keyboard_input(keyboard_input: &Res<Input<KeyCode>>) -> PlayerInput {
    let mut input: PlayerInput = PlayerInput {
        throttle: 0.0,
        steering: 0.0,
    };

    // Update the acceleration based on the keys that are currently pressed
    if keyboard_input.pressed(KeyCode::Up) {
        input.throttle = 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        //input.throttle = -1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        input.steering = 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        input.steering = -1.0;
    }

    return input;
}

fn tick(
    mut player_query: Query<(&mut PlayerCar, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    col: Res<collision::CollisionResource>,
) {
    let delta_time = time.delta_seconds();
    for (mut car, mut transform) in player_query.iter_mut() {
        let input = get_keyboard_input(&keyboard_input);
        let position_2d = transform.translation.xy();

        // Calculate our new velocity, in the direction of our forward.
        car.velocity = car.heading * (input.throttle * 50.0);

        car.update_steering(input, delta_time, position_2d);

        
        car.velocity = car.heading * car.velocity.length();

        transform.translation.x += car.velocity.x * delta_time;
        transform.translation.y += car.velocity.y * delta_time;

        transform.rotation = Quat::from_rotation_z(car.get_rotatation_rads() - (90.0_f32).to_radians());
    }
}
