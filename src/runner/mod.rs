use std::error::Error;

use crate::simulation::SimulationState;

pub mod cpu;

/// Trait for simulation runners
///
/// Simulation runners are responsible for advacing the state of a thermal simulation in time.
pub trait SimRunner {
    /// Advance the simulation by a given ammount of time, with the given timestep
    fn advace_simulation(
        &mut self,
        current_state: &SimulationState,
        advace_time: f64,
        timestep: f64,
    ) -> impl Future<Output = Result<SimulationState, &(dyn Error)>> + Send;
}
