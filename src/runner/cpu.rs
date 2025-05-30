use crate::world::{SimState, SimWorld};

use super::{SimError, SimRunner};

/// Simulation Runner that uses a single CPU thread to execute
pub struct CPUSimRunner {}

impl SimRunner for CPUSimRunner {
    /// Advace the given simulation state in the given world by a given ammount of total time with
    /// a given timestep
    async fn advance_simulation(
        &self,
        world: &SimWorld,
        current_state: &SimState,
        advace_time: f64,
        timestep: f64,
    ) -> Result<SimState, SimError> {
        todo!()
    }
}
