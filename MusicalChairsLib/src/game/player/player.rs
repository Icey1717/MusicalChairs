use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{log, GameOverEvent};

use super::super::collision;

#[cfg(feature = "graphics")]
use super::player_graphics;
pub struct PlayerPlugin;

pub const CAR_SIZE_PX: Vec2 = Vec2 { x: 44.0, y: 74.0 };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerLoadState {
    Setup,
    GraphicsLoaded,
    Finished,
}

const SPAWN_LOCATION: Vec3 = Vec3::new(450.0, 250.0, 1.0);
const DEFAULT_HEADING: Vec2 = Vec2 { x: 1.0, y: 0.0 };
const USE_AI_PLAYER: bool = false;

#[cfg(not(feature = "graphics"))]
fn check_graphics_setup(app: &mut App) {
    app.add_state(PlayerLoadState::GraphicsLoaded);
}

#[cfg(feature = "graphics")]
fn check_graphics_setup(app: &mut App) {
    app.add_plugin(player_graphics::PlayerGraphicsPlugin);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        check_graphics_setup(app);
        app.add_system_set(
            SystemSet::on_update(PlayerLoadState::GraphicsLoaded).with_system(setup),
        )
        .add_system_set(SystemSet::on_update(PlayerLoadState::Finished).with_system(tick));
    }
}

fn setup(mut state: ResMut<State<PlayerLoadState>>, mut commands: Commands) {
    log::log!("Beginning PlayerLoadState::GraphicsLoaded. Spawning players.");

    // draw a sprite from the atlas
    commands.spawn(PlayerCarBundle {
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: SPAWN_LOCATION,
                scale: Vec3::splat(1.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        player_car: PlayerCar {
            heading: DEFAULT_HEADING,
            is_ai: USE_AI_PLAYER,
            ..default()
        },
        ..default()
    });

    state.set(PlayerLoadState::Finished).unwrap();
}

#[derive(Component, Default)]
pub struct PlayerCar {
    pub velocity: Vec2,
    pub front_wheel: Vec2,
    pub back_wheel: Vec2,
    pub heading: Vec2,
    pub input: PlayerInput,
    pub is_ai: bool,
    pub distance: f32,
}

#[derive(Default, Copy, Clone)]
pub struct PlayerInput {
    pub throttle: f32,
    pub steering: f32,
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
    pub fn reset(&mut self) {
        self.velocity = Vec2::ZERO;
        self.front_wheel = Vec2::ZERO;
        self.back_wheel = Vec2::ZERO;
        self.heading = DEFAULT_HEADING;
        self.input = PlayerInput {
            ..Default::default()
        };
        self.distance = 0.0;
    }

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

#[derive(Bundle, Default)]
pub struct PlayerCarBundle {
    player_car: PlayerCar,
    #[bundle]
    sprite: SpriteSheetBundle,
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
) -> bool {
    // Step 1: Determine the new position of the object after applying the motion vector.
    let new_position = transform.translation.xy() + motion;

    let car_col = car.build_collision(new_position);

    // Step 2: Check for collisions at the new position.
    let mut slide = motion.clone();
    let mut collided = false;
    for other in collision_world.precomputed_rectangles.iter() {
        // Check for a collision between the object and the other object.
        let normal = collision::separating_axis_test(
            &collision::PrecomputedRectangle::from_rect(&car_col),
            other,
        );
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

    return collided;
}

const ENGINE_POWER: f32 = 500.0;
const MAX_SPEED_REVERSE: f32 = 250.0;

#[cfg(not(feature = "graphics"))]
fn get_timestep(_time: &Res<Time>) -> f32 {
    0.033
}

#[cfg(feature = "graphics")]
fn get_timestep(time: &Res<Time>) -> f32 {
    time.delta_seconds()
}

pub fn reset_transform(transform: &mut Transform) {
    transform.translation = SPAWN_LOCATION;
}

fn tick(
    mut player_query: Query<(&mut PlayerCar, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    col: Res<collision::CollisionResource>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let delta_time = get_timestep(&time);
    for (mut car, mut transform) in player_query.iter_mut() {
        let input = if car.is_ai {
            car.input
        } else {
            get_keyboard_input(&keyboard_input)
        };

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
        if move_and_slide(&col, &mut car, &mut transform, motion, original_heading) {
            game_over_writer.send(GameOverEvent);
        }

        transform.rotation = Quat::from_rotation_z(car.get_rotatation_rads());

        car.distance += (transform.translation.xy() - position_2d).length();

        if transform.translation.x > 1000.0 || transform.translation.x < 0.0 || transform.translation.y > 1000.0 || transform.translation.y < 0.0 {
            game_over_writer.send(GameOverEvent);
        }
    }
}
