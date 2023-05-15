use nalgebra::{matrix, Vector3};
use rand_distr::{Distribution, Normal};

pub mod pid;
pub mod plot;

#[derive(Copy, Clone, Debug)]
pub struct Config {
    thrust_err: Normal<f64>,
    sensor_err: Normal<f64>,
    rand_force: Normal<f64>,
    thrust: f64,
    hover_thrust_percent: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            thrust_err: Normal::new(0.0, 2.0).unwrap(),
            sensor_err: Normal::new(0.1, 0.5).unwrap(),
            rand_force: Normal::new(0.0, 5.75).unwrap(),
            thrust: 40.0,
            hover_thrust_percent: 0.24,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sim {
    pub config: Config,
    pub input: Input,
    pub generation: usize,
    pub state: Vector3<f64>,
    pub controller: pid::Controller,
}

#[derive(Copy, Clone, Debug)]
pub struct Input {
    //TODO: Move to utilizing types for time; expedite by assuming seconds
    pub time: f64,
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

    pub fn tick(&mut self, input: &Input) {
        let time = input.time;
        let dt = time - self.input.time;
        let thrust_err = self.config.thrust_err.sample(&mut rand::thread_rng());
        let rand_force = self.config.rand_force.sample(&mut rand::thread_rng());
        let thrust_percent = self.controller_to_thrust();

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

    /// Attempt to linearize PID -> thrust
    fn controller_to_thrust(&self) -> f64 {
        let hover = self.config.hover_thrust_percent;
        let output = self.controller.output().clamp(-1.0, 1.0);

        if output < 0.0 {
            hover * -output
        } else {
            (1.0 - hover) * output
        }
    }

    pub fn pos(&self) -> f64 {
        self.state[0]
    }

    pub fn vel(&self) -> f64 {
        let err = self.config.sensor_err.sample(&mut rand::thread_rng());
        self.state[1] + err
    }

    pub fn accl(&self) -> f64 {
        let err = self.config.sensor_err.sample(&mut rand::thread_rng());
        self.state[2] + err
    }
}
