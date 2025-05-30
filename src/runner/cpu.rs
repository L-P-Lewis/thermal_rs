use crate::world::{SimState, SimWorld};

use super::{SimError, SimRunner};

static CELL_KERLEL: [(i8, i8, i8); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

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
        if !world.is_state_valid(current_state) {
            return Err(SimError::SimStateInvalid);
        }
        let mut active_state = current_state.clone();
        let mut remaining_time = advace_time;
        while remaining_time > 0.0 {
            active_state =
                advance_world_state(world, active_state, timestep.min(remaining_time) as f32);
            remaining_time -= timestep;
        }
        return Ok(active_state);
    }
}

fn advance_world_state(world: &SimWorld, mut current_state: SimState, deltatime: f32) -> SimState {
    // Create energy delta vector
    let mut energy_deltas: Vec<f32> = Vec::new();
    energy_deltas.resize(
        world.get_x_size() * world.get_y_size() * world.get_z_size(),
        0.0,
    );

    let current_energies = current_state.get_energies();
    let materials = world.get_materials();
    let mat_map = world.get_material_map();
    let cell_size = world.get_cell_volume();
    let cell_dist = world.get_cell_length();

    for x in 0..world.get_x_size() {
        for y in 0..world.get_y_size() {
            for z in 0..world.get_z_size() {
                let cell_index = world
                    .get_pos_index(x, y, z)
                    .expect("We know we are iterating over positions in the world");
                let cell_material = mat_map[materials[cell_index] as usize];
                let cell_mass = cell_material.density * cell_size;
                let cell_temperature =
                    current_energies[cell_index] / (cell_material.specific_heat * cell_mass);
                let cell_thermal_conductivity =
                    cell_material.get_thermal_conductivity(cell_temperature);
                let cell_energy_delta = energy_deltas
                    .get_mut(cell_index)
                    .expect("Energy delta list is known to be the same size as energy list");

                for neighbor_index in CELL_KERLEL.iter().filter_map(|(dx, dy, dz)| {
                    world.get_ipos_index(
                        x as i128 + *dx as i128,
                        y as i128 + *dy as i128,
                        z as i128 + *dz as i128,
                    )
                }) {
                    let neighbor_material = mat_map[materials[neighbor_index] as usize];
                    let neighbor_mass = neighbor_material.density * cell_size;
                    let neighbor_temperature = current_energies[neighbor_index]
                        / (neighbor_material.specific_heat * neighbor_mass);
                    let neighbor_thermal_conductivity =
                        neighbor_material.get_thermal_conductivity(neighbor_temperature);

                    let effective_thermal_con =
                        (cell_thermal_conductivity * neighbor_thermal_conductivity) / 2.0;

                    // Calculate energy flow and apply to both energy deltas
                    let heat_delta = neighbor_temperature - cell_temperature;
                    *cell_energy_delta +=
                        heat_delta * effective_thermal_con * deltatime * cell_dist;
                }
            }
        }
    }

    current_state.apply_deltas(energy_deltas.into_iter());
    return current_state;
}
