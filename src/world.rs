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
/// // half fill it with water, and build it with a 1cm voxel size
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
    /// Missmatch between resolution of a simulation state and a simulaton world
    StateResolutionMissmatch,
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
    /// Samples the material stats at the voxel closest to the given point, returns None if given
    /// point is out of bounds
    pub fn sample_material(&self, x: f64, y: f64, z: f64) -> Option<&Material> {
        self.get_voxel_material(
            (x / self.cell_size).floor() as usize,
            (y / self.cell_size).floor() as usize,
            (z / self.cell_size).floor() as usize,
        )
    }

    /// Get the material value at a given voxel
    pub fn get_voxel_material(&self, x: usize, y: usize, z: usize) -> Option<&Material> {
        let pos_to_index = |x: usize, y: usize, z: usize| {
            if x < self.x_size && y < self.x_size && z < self.y_size {
                Some(x + y * self.x_size + z * self.x_size * self.y_size)
            } else {
                None
            }
        };
        let world_ind = match pos_to_index(x, y, z) {
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

    /// Gets a simulation state with no thermal energy
    pub fn get_blank_sim_state(&self) -> SimState {
        todo!()
    }

    /// Sets the temperature of a simulation state within a brush. Fails if state has a differnet
    /// resolution or bounds size
    pub fn set_sim_state_temperature(
        &self,
        sim_state: SimState,
        temperature: f64,
        brush: &impl CellIterator,
    ) -> Result<SimState, SimStateOppError> {
        todo!()
    }
}

/// Represents the distribution of thermal energy in a simulation world at a given state in time
pub struct SimState {}

impl SimState {
    /// Samples the thermal energy closest to a given point, returns None if out of bounds
    pub fn sample_energy(&self, x: f64, y: f64, z: f64) -> Option<f64> {
        todo!()
    }
}
