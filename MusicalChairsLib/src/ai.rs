use bevy::app::AppExit;
use bevy::prelude::*;
use entity_gym_rs::agent::{Action, Agent, AgentOps, Featurizable, Obs};

use crate::{
    game::{collision::CollisionResource, player::player::PlayerCar},
    log,
};

pub struct AiPlayer(pub Box<dyn Agent>);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(car_move_agent);
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Action)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub fn car_move_agent(
    mut ai_player: NonSendMut<AiPlayer>,
    collision: ResMut<CollisionResource>,
    mut exit: EventWriter<AppExit>,
    mut players: Query<(&mut PlayerCar, &Transform)>,
) {
    //log::log!("Running agent");

    let player_states: Vec<PlayerState> = players
        .iter()
        .map(|(other_player, transform)| PlayerState {
            x: transform.translation.x as i32,
            y: transform.translation.y as i32,
            x_velocity: other_player.velocity.x as i32,
            y_velocity: other_player.velocity.y as i32,
            x_heading: other_player.heading.x as i32,
            y_heading: other_player.heading.y as i32,
        })
        .collect();

    if let Some((mut player, _player_transform)) = players.iter_mut().next() {
        //log::log!("Creating observer");
        let obs = Obs::new(player.distance)
            .entities(collision.rectangles.iter().map(|rectangle| Rectangle {
                x: rectangle.x,
                y: rectangle.y,
            }))
            .entities(player_states);
        let action = ai_player.0.act::<Direction>(&obs);
        match action {
            Some(dir) => {
                //log::log!("Doing: {:?}", dir);
                for d in dir {
                    match d {
                        Direction::Left => player.input.steering = -1.0,
                        Direction::Right => player.input.steering = 1.0,
                        Direction::Up => player.input.throttle = 1.0,
                        Direction::Down => player.input.throttle = -1.0,
                    }
                }
            }
            None => exit.send(AppExit),
        }
    }
}

#[derive(Featurizable)]
pub struct Rectangle {
    x: i32,
    y: i32,
}

#[derive(Featurizable)]
pub struct PlayerState {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
    x_heading: i32,
    y_heading: i32,
}
