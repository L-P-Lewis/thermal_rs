use crate::{material, material::Material, volume::CellIterator};

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
        // Get world size
        let world_size_x = (self.x_size / resolution).ceil() as u64;
        let world_size_y = (self.y_size / resolution).ceil() as u64;
        let world_size_z = (self.z_size / resolution).ceil() as u64;

        // Initialize material map data
        let mut material_map: Vec<Material> = Vec::new();
        material_map.push(material::BLANK);
        let mut materials: Vec<u8> = Vec::from_iter(std::iter::repeat_n(
            0 as u8,
            (world_size_z * world_size_y * world_size_x) as usize,
        ));

        // Fill in materials from brushes
        for (mat, brush) in self.brush_opperations.iter() {
            // Find index of first instance of material in map, or add it to the map and get index
            // of new value
            let ind: usize = {
                let mut found: Option<usize> = None;
                for i in 0..material_map.len() {
                    if material_map
                        .get(i)
                        .expect("i should be in bounds of material_map")
                        == mat
                    {
                        found = Some(i);
                        break;
                    }
                }
                if let Some(i) = found {
                    i
                } else {
                    material_map.push(mat.clone());
                    material_map.len() - 1
                }
            };

            assert!(
                ind < u8::MAX as usize,
                "A simulation world can not have more than 256 unique materials"
            );

            // Set material index of all cells in brush
            for (x, y, z) in brush.cell_iter(&resolution) {
                if let Some(m) = materials
                    .get_mut((x + y * world_size_x + z * (world_size_x * world_size_y)) as usize)
                {
                    *m = ind as u8;
                }
            }
        }

        // Return new sim world
        return SimWorld {
            x_size: world_size_x as usize,
            y_size: world_size_y as usize,
            z_size: world_size_z as usize,
            cell_size: resolution,
            material_map,
            materials,
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
    pub fn sample_material(&self, x: f64, y: f64, z: f64) -> Option<Material> {
        todo!()
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
