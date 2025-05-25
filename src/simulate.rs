use std::iter;

use crate::{Volume, material::Material};

/// Represents a thermal simulation at a given state in time
#[derive(Debug, Clone)]
pub struct ThermalSimulation {
    /// The number of cells in the x direction
    cells_x: u64,
    /// The number of cells in the y direction
    cells_y: u64,
    /// The number of cells in the z direction
    cells_z: u64,
    /// The side dimension of cells in meters
    cell_size: f64,
    /// Map of material ids to the material data
    material_map: Vec<Material>,
    /// Energy data
    energy: Vec<f64>,
    /// Material data
    materials: Vec<usize>,
}

impl ThermalSimulation {
    /// Creates a new blank thermal simulation
    pub fn new(size_x: f64, size_y: f64, size_z: f64, cell_size: f64) -> ThermalSimulation {
        let cells_x: u64 = ((size_x / cell_size).floor()) as u64;
        let cells_y: u64 = ((size_y / cell_size).floor()) as u64;
        let cells_z: u64 = ((size_z / cell_size).floor()) as u64;

        return ThermalSimulation {
            cells_x,
            cells_y,
            cells_z,
            cell_size,
            material_map: vec![Material::blank()],
            energy: Vec::from_iter(iter::repeat_n(0.0, (cells_x * cells_y * cells_z) as usize)),
            materials: Vec::from_iter(iter::repeat_n(0, (cells_x * cells_y * cells_z) as usize)),
        };
    }

    /// Advances the simulation by a given ammount of time
    pub fn advance_simulation(&mut self, delta_time: f64) {
        todo!()
    }

    /// Sets the material of all cells in a given volume
    pub fn set_material(&mut self, volume: Volume, material: Material) {
        todo!()
    }

    /// Sets the temperature of all cells in a given volume
    pub fn set_temperature(&mut self, volume: Volume, temperature: f64) {
        todo!()
    }

    /// Adds energy to all cells in a given volume
    pub fn add_energy(&mut self, volume: Volume, energy: f64) {
        todo!()
    }
}
