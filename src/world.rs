use std::collections::HashMap;

use crate::{
    material::{self, Material},
    volume::CellIterator,
};

/// A builder for simulation worlds
///
/// Used to create a static simulation world.
///
/// ## Example
/// ```
/// # use thermal_rs::{world::SimWorldBuilder, material, volume::AABBVolume};
/// // Create a new world with a 1 meter cubic sim area
/// // half fill it with water, and build it with a 10cm voxel size
/// let newWorld = SimWorldBuilder::new(1.0, 1.0, 1.0)
///     .with_material(material::WATER, Box::new(AABBVolume::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0)))
///     .build(0.1);
/// ```
#[derive(Default)]
pub struct SimWorldBuilder {
    x_size: f64,
    y_size: f64,
    z_size: f64,
    brush_opperations: Vec<(Material, Box<dyn CellIterator>)>,
}

impl SimWorldBuilder {
    /// Create a new builder, defining the dimensions of the world to be built
    pub fn new(x_size: f64, y_size: f64, z_size: f64) -> Self {
        SimWorldBuilder {
            x_size,
            y_size,
            z_size,
            brush_opperations: Vec::new(),
        }
    }

    /// Applies a material type to a volume defined by a brush
    pub fn with_material(mut self, material: Material, brush: Box<dyn CellIterator>) -> Self {
        self.brush_opperations.push((material, brush));
        return self;
    }

    /// Build the world with a given voxel resolution
    pub fn build(self, resolution: f64) -> SimWorld {
        // Get x y and z size of world in voxels
        let world_x = (self.x_size / resolution).ceil() as usize;
        let world_y = (self.y_size / resolution).ceil() as usize;
        let world_z = (self.z_size / resolution).ceil() as usize;

        let pos_to_index = |x: usize, y: usize, z: usize| {
            if x < world_x && y < world_y && z < world_z {
                Some(x + y * world_x + z * world_x * world_y)
            } else {
                None
            }
        };

        // Create new material buffer
        let mut material_buffer: Vec<u8> = Vec::new();
        material_buffer.resize(world_x * world_y * world_z, 0);

        // Create material map
        let mut material_map: HashMap<Material, u8> = HashMap::from([(material::BLANK, 0)]);

        // Write brushes into buffer
        for (mat, brush) in self.brush_opperations.iter() {
            let index: u8 = match material_map.get(mat) {
                Some(i) => *i as u8,
                None => {
                    let new_index = material_map.len();
                    material_map.insert(mat.clone(), new_index as u8);
                    assert!(
                        new_index <= u8::MAX as usize,
                        "There can be at most 256 distinct materials present in a simulation."
                    );
                    new_index as u8
                }
            };

            for (x, y, z) in brush.cell_iter(resolution) {
                if let Some(i) = pos_to_index(x, y, z) {
                    if let Some(v) = material_buffer.get_mut(i) {
                        *v = index;
                    }
                }
            }
        }

        let mut material_list: Vec<Material> = Vec::new();
        material_list.resize(material_map.len(), material::BLANK);

        for (k, v) in material_map.into_iter() {
            if let Some(list_val) = material_list.get_mut(v as usize) {
                *list_val = k;
            }
        }

        return SimWorld {
            x_size: world_x,
            y_size: world_y,
            z_size: world_z,
            cell_size: resolution,
            material_map: material_list,
            materials: material_buffer,
        };
    }
}

/// Possible errors when operating on sim states
#[derive(Debug)]
pub enum SimStateOppError {
    /// Missmatch between sizes of a simulation state and a simulaton world
    StateSizeMissmatch,
}

/// Represents a world in which a simulation can be run
pub struct SimWorld {
    // The x dimension of the simulation world, in cells
    x_size: usize,
    // The y dimension of the simulation world, in cells
    y_size: usize,
    // The z dimension of the simulation world, in cells
    z_size: usize,
    // The side length of cells in meters
    cell_size: f64,
    // A list of all materials present in the simulation world
    material_map: Vec<Material>,
    // A map of all materials in the world, indexing into the material_map
    materials: Vec<u8>,
}

impl SimWorld {
    /// Get the x dimension of the world
    pub fn get_x_size(&self) -> usize {
        self.x_size
    }

    /// Get the y dimension of the world
    pub fn get_y_size(&self) -> usize {
        self.y_size
    }

    /// Get the z dimension of the world
    pub fn get_z_size(&self) -> usize {
        self.z_size
    }

    /// Gets a non-mutable buffer representing the world cell materials
    pub fn get_materials<'a>(&'a self) -> &'a [u8] {
        self.materials.as_slice()
    }

    /// Gets a non-mutable buffer representing the material map
    pub fn get_material_map<'a>(&'a self) -> &'a [Material] {
        self.material_map.as_slice()
    }

    /// Gets the index of a cell position, returns None if out of bounds
    pub fn get_pos_index(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x < self.x_size && y < self.x_size && z < self.y_size {
            Some(x + y * self.x_size + z * self.x_size * self.y_size)
        } else {
            None
        }
    }

    /// Samples the material stats at the voxel closest to the given point, returns None if given
    /// point is out of bounds
    pub fn sample_material(&self, x: f64, y: f64, z: f64) -> Option<&Material> {
        self.get_voxel_material(
            (x / self.cell_size).floor() as usize,
            (y / self.cell_size).floor() as usize,
            (z / self.cell_size).floor() as usize,
        )
    }

    /// Get the material value at a given voxel. Returns none if voxel is out of bounds.
    pub fn get_voxel_material(&self, x: usize, y: usize, z: usize) -> Option<&Material> {
        let world_ind = match self.get_pos_index(x, y, z) {
            Some(i) => i,
            None => {
                return None;
            }
        };
        let world_value = match self.materials.get(world_ind) {
            Some(i) => *i as usize,
            None => {
                return None;
            }
        };
        return self.material_map.get(world_value);
    }

    /// Gets a simulation state with no thermal energy.
    pub fn get_blank_sim_state(&self) -> SimState {
        let mut energies: Vec<f32> = Vec::new();
        energies.resize(self.x_size * self.y_size * self.z_size, 0.0);
        SimState { energies }
    }

    /// Sets the temperature of a simulation state within a brush. Fails if state has a differnet
    ///  bounds size
    pub fn set_sim_state_temperature(
        &self,
        mut sim_state: SimState,
        temperature: f64,
        brush: &impl CellIterator,
    ) -> Result<SimState, SimStateOppError> {
        if sim_state.energies.len() != self.z_size * self.y_size * self.x_size {
            return Err(SimStateOppError::StateSizeMissmatch);
        }

        let cell_volume = self.cell_size.powf(3.0);
        for index in brush
            .cell_iter(self.cell_size)
            .filter_map(|x| self.get_pos_index(x.0, x.1, x.2))
        {
            let cell_mat_id = self
                .materials
                .get(index)
                .expect("Indicies are pre-verified");
            let cell_material = self
                .material_map
                .get(*cell_mat_id as usize)
                .expect("Cell material IDs are static and must be valid");
            let cell_mass = cell_volume * cell_material.density;
            if let Some(e) = sim_state.energies.get_mut(index) {
                *e = (temperature * cell_mass * cell_material.specific_heat) as f32;
            }
        }
        return Ok(sim_state);
    }

    /// Samples the temperature of a given voxel. Returns None if given position is out of bounds
    /// or simulation state is of the wrong size
    fn sample_voxel_temperature(
        &self,
        sim_state: &SimState,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<f32> {
        if sim_state.energies.len() != self.materials.len() {
            return None;
        }
        if x >= self.x_size || y >= self.y_size || z > self.z_size {
            let index = x + y * self.x_size + z * self.x_size * self.y_size;
            let cell_mat_id = self
                .materials
                .get(index)
                .expect("Indicies are pre-verified");
            let cell_material = self
                .material_map
                .get(*cell_mat_id as usize)
                .expect("Cell material IDs are static and must be valid");
            let cell_energy = sim_state
                .energies
                .get(*cell_mat_id as usize)
                .expect("State is already known to be correct size");
            let cell_mass = self.cell_size.powf(3.0) * cell_material.density;

            return Some(cell_energy / (cell_mass * cell_material.specific_heat) as f32);
        } else {
            return None;
        }
    }
}

/// Represents the distribution of thermal energy in a simulation world at a given state in time
///
/// Has little meaning on it's own, is only usefull in the context of a [SimWorld]
pub struct SimState {
    energies: Vec<f32>,
}
