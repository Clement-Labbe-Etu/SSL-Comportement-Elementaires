use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{OPoint, Point2};
use std::{f64::consts::PI, ptr::null};

/// The goto struct represents a strategy that commands a robot to move in a goto shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct goto {
    /// The id of the robot to move.
    id: u8,
    target: Point2<f64>,
}

impl goto {
    /// Creates a new goto instance with the desired robot id.
    pub fn new(id: u8, target: Point2<f64>) -> Self {
        Self { id , target }
    }
}

impl Strategy for goto {
    fn name(&self) -> &'static str {
        "goto"
    }

    /// Executes the goto strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a goto shape in a
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
        println!("Atomique : Goto");
        action_wrapper.push(
            self.id,
            MoveTo::new(self.target, 0.0, 0.0, false, None),
        );
        
        true
    }
}
