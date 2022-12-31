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
                SystemSet::on_update(PlayerLoadState::Finished).with_system(update_debug_sprites),
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

const MAX_STEERING: f32 = std::f32::consts::PI / 4.0;

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

    const STOPPING_VELOCITY: f32 = 5.0;
    const FRICTION: f32 = -0.9;
    const DRAG: f32 = -0.0015;

    fn apply_friction(&mut self, acceleration: &mut Vec2) {
        if self.velocity.length() < PlayerCar::STOPPING_VELOCITY {
            self.velocity = Vec2::ZERO;
        }

        let mut friction_force = self.velocity * PlayerCar::FRICTION;
        let drag_force = self.velocity * self.velocity.length() * PlayerCar::DRAG;
        if self.velocity.length() < 100.0 {
            friction_force *= 3.0;
        }

        *acceleration += drag_force + friction_force;
    }

    fn get_rotatation_rads(&self) -> f32 {
        Vec2::new(1.0, 0.0).angle_between(self.heading) - (90.0_f32).to_radians()
    }

    fn build_collision(&self, position: Vec2) -> collision::Rectangle {
        collision::Rectangle {
            x: position.x as i32,
            y: position.y as i32,
            width: CAR_SIZE_PX.x as i32,
            height: CAR_SIZE_PX.y as i32,
            rotation: self.get_rotatation_rads() as f64,
        }
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
                    "v: x: {:.1}, y: {:.1}\np: x: {:.1}, y: {:.1}",
                    car.velocity.x,
                    car.velocity.y,
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

fn update_debug_sprites(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_sprite_query: Query<(&PlayerDebug, &mut Transform, Without<PlayerCar>)>,
) {
    let mut must_be_front = false;
    for (player_debug, mut sprite_transform, ()) in player_sprite_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            if must_be_front {
                sprite_transform.translation.x = car.front_wheel.x;
                sprite_transform.translation.y = car.front_wheel.y;
            } else {
                sprite_transform.translation.x = car.back_wheel.x;
                sprite_transform.translation.y = car.back_wheel.y;
            }

            sprite_transform.translation.z = car_transform.translation.z + 1.0;
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
        input.throttle = -1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        input.steering = 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        input.steering = -1.0;
    }

    return input;
}

fn move_and_slide(
    collision_world: &Res<collision::CollisionResource>,
    car: &mut PlayerCar,
    transform: &mut Transform,
    motion: Vec2,
    heading: Vec2,
) {
    // Step 1: Determine the new position of the object after applying the motion vector.
    let new_position = transform.translation.xy() + motion;

    let car_col = car.build_collision(new_position);

    // Step 2: Check for collisions at the new position.
    let mut slide = motion.clone();
    let mut collided = false;
    for other in collision_world.precomputed_rectangles.iter() {
        // Check for a collision between the object and the other object.
        let normal = collision::separating_axis_test(&collision::PrecomputedRectangle::from_rect(&car_col), other);
        if normal.is_some() {
            collided = true;
            // Find the slide vector by reflecting the motion vector over the normal of the collision surface.
            let normal = Vec2::new(normal.unwrap().x as f32, normal.unwrap().y as f32);
            slide = slide - slide.dot(normal) * normal;
        }
    }

    // Step 3: If there was a collision, apply the slide vector to the object's position.
    if collided {
        //transform.translation.x += slide.x;
        //transform.translation.y += slide.y;
        car.heading = heading;
        car.velocity = Vec2::new(0.0, 0.0);
    } else {
        // If there was no collision, apply the original motion vector to the object's position.
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}

const ENGINE_POWER: f32 = 500.0;
const MAX_SPEED_REVERSE: f32 = 250.0;

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

        let mut acceleration = car.heading * (input.throttle * ENGINE_POWER);

        let original_heading = car.heading;

        car.apply_friction(&mut acceleration);

        car.velocity += acceleration * delta_time;

        car.update_steering(input, delta_time, position_2d);

        let d = car.heading.dot(car.velocity.normalize());
        if d > 0.0 {
            car.velocity = car.heading * car.velocity.length();
        }
        if d < 0.0 {
            car.velocity = -car.heading * car.velocity.length().min(MAX_SPEED_REVERSE);
        }

        let motion = car.velocity * delta_time;
        move_and_slide(&col, &mut car, &mut transform, motion, original_heading);

        transform.rotation = Quat::from_rotation_z(car.get_rotatation_rads());
    }
}
