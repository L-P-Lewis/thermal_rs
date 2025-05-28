use super::SimRunner;

/// Simulation Runner that uses a single CPU thread to execute
pub struct CPUSimRunner {}

impl SimRunner for CPUSimRunner {
    /// Advace the given simulation state in the given world by a given ammount of total time with
    /// a given timestep
    fn advace_simulation(
        &mut self,
        world: &crate::world::SimWorld,
        current_state: &crate::world::SimState,
        advace_time: f64,
        timestep: f64,
    ) -> impl Future<Output = Result<crate::world::SimWorld, &(dyn std::error::Error)>> + Send {
        async { todo!() }
    }
}
