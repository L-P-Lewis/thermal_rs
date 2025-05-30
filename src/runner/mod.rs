use std::{error::Error, future::Future};

use crate::world::{SimState, SimWorld};

/// Single Threaded CPU based simulator
pub mod cpu;

#[derive(Debug)]
/// Simulation Runtime Error
pub enum SimError {
    /// The simulation state passed in is not a valid state of the simulatin
    SimStateInvalid,
}

/// Trait for simulation runners
///
/// Simulation runners are responsible for advacing the state of a thermal simulation in time.
pub trait SimRunner {
    /// Advance the simulation by a given ammount of time, with the given timestep
    fn advance_simulation(
        &self,
        world: &SimWorld,
        current_state: &SimState,
        advace_time: f64,
        timestep: f64,
    ) -> impl std::future::Future<Output = Result<SimState, SimError>> + Send;
}
