use std::time::Instant;

use crate::state::TrafficLightColor;

pub struct TrafficLightApp {
    color_a: TrafficLightColor,
    color_b: TrafficLightColor,
    simulation_complete: bool,
    last_update: Instant
}

impl TrafficLightApp {
    fn run_simulation_step(&mut self) {
        if self.simulation_complete {
            return;
        }

        let delta_time = self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();
    }
}