use crate::{material::Material, volume::CellIterator};

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
///     .build(0.001);
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
        todo!()
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
pub struct SimWorld {}

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
