#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub p: f64,
    pub i: f64,
    pub d: f64,
}

/// Classic PID
#[derive(Clone, Copy, Debug)]
pub struct Controller {
    pub config: Config,
    pub generation: usize,
    pub time: f64,
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
    pub fn step(&self, input: &Input) -> Self {
        let dt = input.time - self.time;
        let error = input.setpoint - input.measured;
        let integral = self.integral + error * dt;
        let derivative = if self.generation == 0 {
            0.0
        } else {
            (error - self.error) / dt
        };

        Self {
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
            time: 0.0,
            error: 0.0,
            integral: 0.0,
            derivative: 0.0,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            p: 0.3,
            i: 0.0,
            d: 150.0,
        }
    }
}
