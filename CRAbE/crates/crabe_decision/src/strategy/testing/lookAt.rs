use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{OPoint, Point2};
use std::{f64::consts::PI, ptr::null};

/// The look_at struct represents a strategy that commands a robot to move in a look_at shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct look_at {
    /// The id of the robot to move.
    id: u8,
    target: Point2<f64>,
}

fn look_at_target(robot: Point2<f64>, target: Point2<f64>) -> f64 {
    let diff_x = target.x - robot.x;
    let diff_y = target.y - robot.y;
    diff_y.atan2(diff_x)
}

impl look_at {
    /// Creates a new look_at instance with the desired robot id.
    pub fn new(id: u8, target: Point2<f64>) -> Self {
        Self { id , target }
    }
}

impl Strategy for look_at {
    fn name(&self) -> &'static str {
        "look_at"
    }

    /// Executes the look_at strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a look_at shape in a
    /// counter-clockwise direction.
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    #[allow(unused_variables)]

    
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false;
            }
        }
        .pose;

        println!("Atomique : look_at");
        action_wrapper.push(
            self.id,
            MoveTo::new(robot.position, look_at_target(robot.position , self.target), 0.0, false, None),
        );

        true
    }
}
