use std::error::Error;

use crate::simulation::SimulationState;

/// Trait for simulation runners
///
/// Simulation runners are responsible for advacing the state of a thermal simulation in time.
pub trait SimRunner {
    /// Sets the active state of the simulation
    fn set_simulation_state(&mut self, state: &SimulationState) -> Result<(), &(dyn Error)>;
    /// Gets the active state of the simulation
    fn get_simulation_state(&self) -> Result<SimulationState, &(dyn Error)>;
    /// Advance the simulation by a given ammount of time, with the given timestep
    fn advace_simulation(
        &mut self,
        advace_time: f64,
        timestep: f64,
    ) -> impl Future<Output = Result<(), &(dyn Error)>> + Send;
}
