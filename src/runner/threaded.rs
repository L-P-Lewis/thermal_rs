use std::{
    clone,
    sync::{Arc, Mutex, mpsc::channel},
    usize,
};

use threadpool::ThreadPool;

use crate::{
    material::Material,
    world::{SimState, SimWorld},
};

use super::{SimError, SimRunner};

/// A multithreaded cpu based simulation runner
pub struct ThreadedRunner {
    workers: usize,
    chunk_size: usize,
}

impl ThreadedRunner {
    /// Create a new threaded runner with the given num threads and chunk size
    pub fn new(workers: usize, chunk_size: usize) -> Self {
        Self {
            workers,
            chunk_size,
        }
    }
}

impl SimRunner for ThreadedRunner {
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
        let materials = Arc::new(Vec::from(world.get_materials()).as_slice());
        let material_map = Arc::new(Vec::from(world.get_material_map()).as_slice());
        let mut remaining_time = advace_time;
        /// Create local copy of world for async reasons
        let world = world.clone();
        while remaining_time > 0.0 {
            active_state = advance_world_state(
                materials.clone(),
                material_map.clone(),
                (world.get_x_size(), world.get_y_size(), world.get_z_size()),
                active_state,
                timestep.min(remaining_time) as f32,
                self.chunk_size,
                self.workers,
            )
            .await;
            remaining_time -= timestep;
        }
        return Ok(active_state);
    }
}

async fn advance_world_state(
    materials: Arc<&[u8]>,
    material_map: Arc<&[Material]>,
    (world_x, world_y, world_z): (usize, usize, usize),
    mut current_state: SimState,
    deltatime: f32,
    chunk_size: usize,
    worker_count: usize,
) -> SimState {
    // Create energy delta vector
    let mut energy_deltas: Vec<f32> = Vec::new();
    energy_deltas.resize(world_x * world_y * world_z, 0.0);

    let x_chunks = world_x / chunk_size;
    let x_remainder = world_x - (chunk_size * x_chunks);
    let y_chunks = world_y / chunk_size;
    let y_remainder = world_y - (chunk_size * y_chunks);
    let z_chunks = world_z / chunk_size;
    let z_remainder = world_z - (chunk_size * z_chunks);

    let pos_to_index = |x: usize, y: usize, z: usize| {
        x + y * world_x.clone() + z * world_x.clone() * world_y.clone()
    };

    let current_energies = Arc::new(current_state.get_energies());
    let energy_delt_mut = Arc::new(Mutex::new(energy_deltas.as_mut_slice()));
    let pool = ThreadPool::new(worker_count);

    for x in 0..=x_chunks {
        for y in 0..=y_chunks {
            for z in 0..=z_chunks {
                let xmin = chunk_size * x;
                let ymin = chunk_size * y;
                let zmin = chunk_size * z;
                let xsize = if x < x_chunks {
                    chunk_size
                } else {
                    x_remainder
                };
                let ysize = if y < y_chunks {
                    chunk_size
                } else {
                    y_remainder
                };
                let zsize = if z < z_chunks {
                    chunk_size
                } else {
                    z_remainder
                };
                let pos_to_index = pos_to_index.clone();
                let materials = materials.clone();
                let material_map = material_map.clone();
                let current_energies = current_energies.clone();
                let energy_deltas = energy_deltas.clone();
                pool.execute(move || {
                    for x in xmin..(xmin + xsize) {
                        for y in ymin..(ymin + ysize) {
                            for x in zmin..(zmin + zsize) {
                                let mut cell_delta = 0.0;
                                cell_delta += get_energy_flow(
                                    pos_to_index(x, y, z),
                                    pos_to_index(x + 1, y, z),
                                    materials,
                                    material_map,
                                    current_energies,
                                );
                            }
                        }
                    }
                });
            }
        }
    }

    current_state.apply_deltas(energy_deltas.into_iter());
    return current_state;
}

fn get_energy_flow(
    from: usize,
    to: usize,
    materials: Arc<&[u8]>,
    material_map: Arc<&[Material]>,
    energies: Arc<&[f32]>,
) -> f32 {
    todo!()
}
