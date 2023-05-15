use sim::{plot, Input, Sim};

struct Config {
    time_step: f64,
    duration: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            time_step: 0.01,
            duration: 20.0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::default();
    let iterations = (cfg.duration / cfg.time_step) as usize;
    let states: Vec<Sim> = (0..iterations)
        .scan(Sim::default(), |state, i| {
            let time = i as f64 * cfg.time_step;
            let input = Input {
                time,
                setpoint: if time < cfg.duration / 2.0 { 50.0 } else { 100.0 },
            };
            *state = state.step(&input);
            Some(*state)
        })
        .collect();

    let plots = vec![
        plot::plot(
            "Sim State",
            vec![plot::lines(
                &states,
                &[
                    ("Position", |x| x.input.time, |x| x.pos()),
                    ("Velocity", |x| x.input.time, |x| x.vel()),
                    // ("Acceleration", |x| x.input.time, |x| x.accl()),
                    ("Setpoint", |x| x.input.time, |x| x.input.setpoint),
                ],
            )]
            .into_iter()
            .flatten()
            .collect(),
        ),
        plot::plot(
            "Controller",
            vec![plot::lines(
                &states,
                &[
                    ("Error", |x| x.input.time, |x| x.controller.error),
                    // ("Integral", |x| x.input.time, |x| x.controller.integral),
                    // ("Derivative", |x| x.input.time, |x| x.controller.derivative),
                    ("Output", |x| x.input.time, |x| x.controller_to_thrust()),
                ],
            )]
            .into_iter()
            .flatten()
            .collect(),
        ),
    ];

    plot::show(&plots);

    Ok(())
}
