use crate::{runner::SimRunner, simulation::SimulationState};

/// Simulation Runner that uses a single CPU thread to execute
pub struct CPUSimRunner {
    sim_state: SimulationState,
}

impl CPUSimRunner {
    fn do_timestep(&mut self, deltatime: f64) {
        let mut energy_deltas: Vec<f64> = vec![];
        todo!()
    }
}

impl SimRunner for CPUSimRunner {
    fn advace_simulation(
        &mut self,
        current_state: &SimulationState,
        advace_time: f64,
        timestep: f64,
    ) -> impl Future<Output = Result<SimulationState, &(dyn std::error::Error)>> + Send {
        let full_steps = (advace_time / timestep).floor() as u64;
        let remainder = advace_time - (full_steps as f64 * timestep);

        for _ in 0..full_steps {
            self.do_timestep(timestep);
        }

        if remainder > 0.0 {
            self.do_timestep(remainder);
        }

        async { todo!() }
    }
}
