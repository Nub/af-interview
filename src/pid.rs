/// This module implements a simple PID controller algorithm in a purely functional manner

/// Configuration for a PID controller
/// In a more complex this would include gains, limits, scheduling, filtering etc...
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Traditionally known as ki
    pub p: f64,
    /// Traditionally known as kp
    pub i: f64,
    /// Traditionally known as kd
    pub d: f64,
}

/// The current state of the PID controller
/// Captures all required data to replay the state
#[derive(Clone, Copy, Debug)]
pub struct Controller {
    pub config: Config,
    pub generation: usize,
    pub input: Input,
    pub error: f64,
    pub integral: f64,
    pub derivative: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub time: f64,
    pub setpoint: f64,
    pub measured: f64,
}

impl Controller {
    /// Pure function to evaluate the next PID controller state given the previous and a new input
    /// Captures required inputs to be replayed
    pub fn step(&self, input: &Input) -> Self {
        let dt = input.time - self.input.time;
        let error = input.setpoint - input.measured;
        let integral = self.integral + error * dt;
        let derivative = (error - self.error) / dt;

        Self {
            input: *input,
            generation: self.generation + 1,
            error,
            integral,
            derivative,
            ..*self
        }
    }

    pub fn output(&self) -> f64 {
        self.config.p * self.error + self.config.i * self.integral + self.config.d * self.derivative
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            config: Config::default(),
            generation: 0,
            input: Input {
                time: 0.0,
                setpoint: 0.0,
                measured: 0.0,
            },
            error: 0.0,
            integral: 0.0,
            derivative: 0.0,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            p: 0.075,
            i: 0.0015,
            d: 0.1,
        }
    }
}
