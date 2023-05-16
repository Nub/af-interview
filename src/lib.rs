use nalgebra::{matrix, Vector3};
use rand_distr::{Distribution, Normal};

pub mod pid;
pub mod plot;

/// The physics simulation configuration
#[derive(Copy, Clone, Debug)]
pub struct Config {
    thrust_err: Normal<f64>,
    sensor_err: Normal<f64>,
    rand_force: Normal<f64>,
    thrust: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thrust_err: Normal::new(0.0, 2.0).unwrap(),
            sensor_err: Normal::new(0.1, 0.5).unwrap(),
            rand_force: Normal::new(0.0, 5.75).unwrap(),
            thrust: 40.0,
        }
    }
}

/// The physics/controller simulation state for a single iteration
#[derive(Copy, Clone, Debug)]
pub struct Sim {
    pub config: Config,
    pub input: Input,
    pub generation: usize,
    pub state: Vector3<f64>,
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

impl Default for Sim {
    fn default() -> Self {
        Self {
            config: Config::default(),
            input: Input {
                time: 0.0,
                setpoint: 50.0,
            },
            generation: 0,
            state: Vector3::new(0.0, 0.0, 0.0),
            controller: pid::Controller::default(),
        }
    }
}

impl Sim {
    /// Run the physics simulation and controller
    /// Wrapped into a pure function
    pub fn step(&self, input: &Input) -> Self {
        let mut prev = *self;

        prev.tick(input);

        let controller = self.controller.step(&pid::Input {
            time: input.time,
            setpoint: input.setpoint,
            measured: self.pos(),
        });

        Self {
            input: *input,
            generation: self.generation + 1,
            controller,
            ..prev
        }
    }

    /// The physics simulation altered to accept a shared input structure.
    // TODO: make this a stateless pure function
    pub fn tick(&mut self, input: &Input) {
        let time = input.time;
        let dt = time - self.input.time;
        let thrust_err = self.config.thrust_err.sample(&mut rand::thread_rng());
        let rand_force = self.config.rand_force.sample(&mut rand::thread_rng());
        let thrust_percent = self.controller_to_thrust().clamp(0.0,1.0);

        *self.state.index_mut((2, 0)) =
            self.config.thrust * thrust_percent + rand_force + thrust_err - 9.81;

        self.state = matrix![
            1.0, dt, 0.5 * dt.powf(2.0);
            0.0, 1.0, dt;
            0.0, 0.0, 1.0
        ] * self.state;
        let y = self.state.index_mut((0, 0));
        *y = y.max(0.0);
        if *y == 0.0 {
            let v = self.state.index_mut((1, 0));
            *v = v.max(0.0);
        }
    }

    /// This is meant to capture additional vehicle dynamics yet to be modeled
    /// Such as thrust linearization, for now we just clamp the controller outputs to some sane
    /// range
    pub fn controller_to_thrust(&self) -> f64 {
        self.controller.output().clamp(-1.0, 1.0)
    }

    /// Getter for position
    pub fn pos(&self) -> f64 {
        self.state[0]
    }

    /// Getter for velocity
    pub fn vel(&self) -> f64 {
        let err = self.config.sensor_err.sample(&mut rand::thread_rng());
        self.state[1] + err
    }

    /// Getter for acceleration
    pub fn accl(&self) -> f64 {
        let err = self.config.sensor_err.sample(&mut rand::thread_rng());
        self.state[2] + err
    }
}
