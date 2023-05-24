use nalgebra::{matrix, Vector3};
use rand_distr::{Distribution, Normal};

pub mod pid;
pub mod plot;

// This is the original lib.rs just renamed
pub mod sim;
use sim::Vehicle;

/// The physics/controller simulation state for a single iteration
#[derive(Copy, Clone, Debug)]
pub struct Sim {
    pub input: Input,
    pub generation: usize,
    pub vehicle: Vehicle,
    pub controller: pid::Controller,
}

/// The variable input (impurities) into the simulation
/// Note: this does not capture the thread local random variable generation
///     this will need to be refactored
#[derive(Copy, Clone, Debug)]
pub struct Input {
    /// Assumed seconds
    // TODO: use typed time
    pub time: f64,
    /// Position the controller will drive to
    // TODO: use typed position
    pub setpoint: f64,
}


impl Sim {
    /// Run the physics simulation and controller
    /// Wrapped into a pure function
    pub fn step(&self, input: &Input) -> Self {
        // Run the control loop on the previously calculated state of the vehicle
        let controller = self.controller.step(&pid::Input {
            time: input.time,
            setpoint: input.setpoint,
            measured: self.vehicle.pos(),
        });

        let dt = input.time - self.input.time;
        // Update the vehicle with the control loop output
        // TODO: remove mutability, skirted around for now with duplication
        let mut vehicle = self.vehicle.clone();
        vehicle.tick(controller.output(), dt); 

        // Produce this state of the simulation
        Self {
            input: *input,
            generation: self.generation + 1,
            vehicle,
            controller,
            ..*self
        }
    }
}

impl Default for Sim {
    fn default() -> Self {
        Self {
            input: Input {
                time: 0.0,
                setpoint: 50.0,
            },
            generation: 0,
            vehicle: Vehicle::default(),
            controller: pid::Controller::default(),
        }
    }
}

