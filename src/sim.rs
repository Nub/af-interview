use nalgebra::{matrix, Vector3};
use rand_distr::{Distribution, Normal};

#[derive(Copy, Clone, Debug)]
pub struct Vehicle {
    state: Vector3<f64>,
    thrust_err: Normal<f64>,
    sensor_err: Normal<f64>,
    rand_force: Normal<f64>,
    thrust: f64,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            state: Default::default(),
            thrust_err: Normal::new(0.0, 2.0).unwrap(),
            sensor_err: Normal::new(0.1, 0.5).unwrap(),
            rand_force: Normal::new(0.0, 5.75).unwrap(),
            thrust: 40.0,
        }
    }
}

impl Vehicle {
    pub fn tick(&mut self, thrust_percent: f64, dt: f64) {
        let thrust_err = self.thrust_err.sample(&mut rand::thread_rng());
        let rand_force = self.rand_force.sample(&mut rand::thread_rng());
        let thrust_percent = thrust_percent.clamp(0.0, 1.0);
        *self.state.index_mut((2, 0)) =
            self.thrust * thrust_percent + rand_force + thrust_err - 9.81;
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

    pub fn velocity(&self) -> f64 {
        let err = self.sensor_err.sample(&mut rand::thread_rng());
        *self.state.index((1, 0)) + err
    }

    pub fn pos(&self) -> f64 {
        *self.state.index((0, 0))
    }

    pub fn accl(&self) -> f64 {
        let err = self.sensor_err.sample(&mut rand::thread_rng());
        *self.state.index((2, 0)) + err
    }
}
