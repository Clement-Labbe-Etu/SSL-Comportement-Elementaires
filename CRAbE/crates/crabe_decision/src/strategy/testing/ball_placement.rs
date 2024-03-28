use crate::{
    action::{move_to::MoveTo, order_raw::RawOrder, ActionWrapper},
    strategy::Strategy,
};
use crabe_framework::data::{
    output::{Command, Kick::StraightKick},
    tool::ToolData,
    world::{Ball, Pose, World},
};
use log::{error, info};
use nalgebra::{ComplexField, Point2};
use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

const DISTANCE_TO_BALL: f64 = 0.05;
const INACURACY: f64 = 0.01;


/// The BallPlacement struct represents a strategy that commands a robot to middle of the ground
pub struct BallPlacement {
    /// The id of the robot to move.
    id: u8,
    first: bool,
    start_time: Instant,
    state: BallStatePlacement,
}

#[derive(PartialEq, Eq)]
pub enum BallStatePlacement {
    GO_BALL,
    DRIBBLE,
    TURN,
    GO,
    STOP,
    
}



fn look_at_target(robot: Point2<f64>, target: Point2<f64>) -> f64 {
    let diff_x = target.x - robot.x;
    let diff_y = target.y - robot.y;
    diff_y.atan2(diff_x)
}

fn get_ball(robot: &Pose, ball: Point2<f64>,imprecis : f64) -> bool {
    let new_pos = (robot.position - ball).normalize()*DISTANCE_TO_BALL;
    let angle = look_at_target(robot.position, ball);
    let new_pos = Point2::new(new_pos.x + ball.x, new_pos.y + ball.y);
    return (new_pos - robot.position).norm() <= (DISTANCE_TO_BALL+INACURACY*imprecis) && (angle.abs() - robot.orientation.abs()).abs() <= (INACURACY*2.0)
}



impl BallPlacement {
    /// Creates a new BallPlacement instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self {
            id,
            first: true,
            start_time: Instant::now(),
            state: BallStatePlacement::GO_BALL,
            
        }
    }
}

impl Strategy for BallPlacement {
    fn name(&self) -> &'static str {
        "BallPlacement"
    }

    /// Executes the BallPlacement strategy.
    ///
    /// This strategy commands the robot with the specified ID to move to the middle of the ground
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
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                eprintln!("Cannot find ball");
                return false;
            }
        }
        .position_2d();

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false;
            }
        }
        .pose;
        let current_time = Instant::now();
        match self.state {
            BallStatePlacement::GO_BALL => {

                

                if get_ball(robot,ball,3.0) {
                        self.start_time = Instant::now();
                        self.state = BallStatePlacement::TURN;
                } 
                else {                  
                    let angle = look_at_target(robot.position, ball);
                    action_wrapper.clear(self.id);
                    action_wrapper.push(self.id,  MoveTo::new(ball, angle , 0.0 , false , Some(StraightKick { power: 0.0 }) ));
                }

                

            }


            BallStatePlacement::DRIBBLE => {
                action_wrapper.push(
                    self.id,
                    RawOrder::new(Command {
                        dribbler: 1.0,
                        ..Default::default()
                    }),
                );
                println!("testttttt");

                if current_time.duration_since(self.start_time) >= Duration::from_secs(1) {           
                    println!("test 2");
                    self.start_time = Instant::now();

                    self.state = BallStatePlacement::TURN;
                }
            }
            BallStatePlacement::TURN => {
                
                let target = Point2::new(1.5,0.0);
                let orient_target = look_at_target(robot.position, target);
                if(robot.orientation <= orient_target+INACURACY*2.0 && robot.orientation >= orient_target+INACURACY*(-2.0) ){
                    self.state = BallStatePlacement::GO;
                } else {
                        action_wrapper.push(
                            self.id,
                            RawOrder::new(Command {
                                forward_velocity: 0.1,
                                left_velocity: 0.0,
                                angular_velocity: 0.2,
                                charge: false,
                                dribbler: 1.0,
                                ..Default::default()
                            }),
                        );
                }

            }

            BallStatePlacement::GO => {

                if  !get_ball(robot,ball,30.0){
                    self.state = BallStatePlacement::GO_BALL;
                } else {


                let target = Point2::new(1.5,0.0);
                let orient_target = look_at_target(robot.position, target);

                if(target-ball).norm() <= INACURACY*3.0{
                    self.start_time = Instant::now();
                    self.state = BallStatePlacement::STOP;
                } else {

                    if !(robot.orientation <= orient_target+INACURACY*3.0 && robot.orientation >= orient_target+INACURACY*(-3.0) ){
                        action_wrapper.push(self.id,  MoveTo::new(robot.position, orient_target , 1.0 , false , Some(StraightKick { power: 0.0 }) ));
                    } else {
                        action_wrapper.push(
                            self.id,
                            RawOrder::new(Command {
                                forward_velocity: 0.4,
                                left_velocity: 0.0,
                                angular_velocity: 0.0,
                                charge: false,
                                dribbler: 1.0,
                                ..Default::default()
                            }),
                        );
                    }
                    
                        
                }

            }
            }

            BallStatePlacement::STOP => {
                if current_time.duration_since(self.start_time) < Duration::from_secs(2){
                    action_wrapper.push(
                        self.id,
                        RawOrder::new(Command {
                            forward_velocity: 0.0,
                            left_velocity: 0.0,
                            angular_velocity: 0.0,
                            charge: false,
                            dribbler: 1.0,
                            ..Default::default()
                        }),
                    );


                } else {
                    action_wrapper.push(
                        self.id,
                        RawOrder::new(Command {
                            ..Default::default()
                        }),
                    );
                    return true;
                }
                
                
            }


        } // match


        false
 
    }
}

