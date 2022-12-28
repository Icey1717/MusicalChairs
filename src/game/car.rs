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
            .add_system_set(SystemSet::on_update(PlayerLoadState::Finished).with_system(update_debug));
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
            text: Text2dBundle {
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
    debug.player_entities.push(commands
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
                ..default()
            },
            ..default()
        })
        .id());
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
    heading: f32,
    size: Vec2,
    wheel_base: f32,
    steer_angle: f32,
}

#[derive(Bundle)]
struct PlayerDebugBundle {
    text: Text2dBundle,
    debug: PlayerDebug,
}

#[derive(Bundle, Default)]
pub struct PlayerCarBundle {
    player_car: PlayerCar,
    #[bundle]
    sprite: SpriteSheetBundle,
}

fn update_debug(
    mut player_query: Query<(&PlayerCar, &Transform)>,
    mut player_text_query: Query<(&PlayerDebug, &mut Text, &mut Transform, Without<PlayerCar>)>,
    debug : Res<PlayerDebugResource>,
) {
    for (player_debug, mut text, mut text_transform, ()) in player_text_query.iter_mut() {
        if let Ok((car, car_transform)) = player_query.get_mut(player_debug.id) {
            // do something with the components
            *text = Text::from_sections([
                TextSection::new(
                    format!("v: x: {:.1}, y: {:.1}\na: x: {:.1}, y: {:.1}\ns: {:.1}\np: x: {:.1}, y: {:.1}", 
                    car.velocity.x, car.velocity.y, 
                    car.acceleration.x, car.acceleration.y,
                    car.heading,
                    car_transform.translation.x, car_transform.translation.y),
                    debug.style.clone(),
                ),
            ]).with_alignment(TextAlignment::CENTER);
            *text_transform = *car_transform;
            text_transform.translation.z += 1.0;
        }
    }
}

fn tick(
    mut player_query: Query<(&mut PlayerCar, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    col: Res<collision::CollisionResource>,
) {
    let delta_time = time.delta_seconds();
    for (mut car, mut transform) in player_query.iter_mut() {
        let mut a = car.acceleration;
        car.steer_angle = 0.0;
        // Update the acceleration based on the keys that are currently pressed
        if keyboard_input.pressed(KeyCode::Up) {
            a.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            a.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            car.steer_angle = -0.1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            car.steer_angle = 0.1;
        }

        //// Normalize the acceleration so that the car doesn't move faster diagonally
        //let b = a.try_normalize();
        //if !b.is_none() {
        //    a = b.unwrap();
        //}

        //// Rotate the acceleration vector by the steering angle

        car.acceleration = a;

        //// Update the velocity based on the acceleration and delta time
        car.velocity += a * delta_time;

        //// Clamp the velocity to the max speed
        //let speed = car.velocity.length();
        //if speed > car.max_speed {
        //    car.velocity = car.velocity * (car.max_speed / speed);
        //}

        //// Save the old position so we can restore it if there's a collision
        //let old_position = transform.translation;

        //// Update the position based on the velocity and delta time
        //transform.translation.x += car.velocity.x * delta_time;
        //transform.translation.y += car.velocity.y * delta_time;

        let position_2d = transform.translation.xy();

        let car_angle: Vec2 = vec2(car.heading.cos() , car.heading.sin());
        let car_steer: Vec2 = vec2((car.heading + car.steer_angle).cos() , (car.heading + car.steer_angle).sin());

        let mut front_wheel = position_2d + car.wheel_base/2.0 * car_angle;
        let mut back_wheel = position_2d - car.wheel_base/2.0 * car_angle;

        front_wheel += car.velocity.x * delta_time * car_steer;
        back_wheel += car.velocity.x * delta_time * car_angle;

        let new_pos =(front_wheel + back_wheel) / 2.0;

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;

        let tan_y = front_wheel.y - back_wheel.y;

        //car.heading = tan_y.atan2(front_wheel.x - back_wheel.x);
        //car.heading = (front_wheel.x - back_wheel.x).atan2(tan_y);

        //// Check for collisions with the obstacles
        //let pos = transform.translation.xy();
        //let rect = Rect::new(
        //    pos.x - (car.size.x / 2.),
        //    pos.y - (car.size.y / 2.),
        //    pos.x + (car.size.x / 2.),
        //    pos.y + (car.size.y / 2.),
        //);
        //for rectangle in col.rectangles.iter() {
        //    let obstacle = rectangle.to_rect();
        //    if !rect.intersect(obstacle).is_empty() {
        //        let normal = (rect.center() - obstacle.center()).normalize();
        //        // There's a collision, so restore the old position
        //        let delta = normal * (obstacle.size().dot(normal) * 0.5 + 0.01);
        //        let new_trans = old_position.xy() + delta;
        //        transform.translation.x = new_trans.x;
        //        transform.translation.y = new_trans.y;
        //        //transform.translation = old_position;
        //        break;
        //    }
        //}
    }
}
