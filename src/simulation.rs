use std::{iter, usize};

use crate::{material::Material, volume::CellIterator};

/// Represents a thermal simulation at a given state in time
#[derive(Debug, Clone)]
pub struct SimulationState {
    /// The number of cells in the x direction
    cells_x: u64,
    /// The number of cells in the y direction
    cells_y: u64,
    /// The number of cells in the z direction
    cells_z: u64,
    /// The side dimension of cells in meters
    cell_size: f64,
    /// Energy data
    energy: Vec<f64>,
    /// Density data
    density: Vec<f64>,
    /// Specific Heat data
    specific_heat: Vec<f64>,
    /// Thermal Conductivity data
    thermal_conductivity: Vec<f64>,
}

impl SimulationState {
    /// Creates a new blank thermal simulation
    pub fn new(size_x: f64, size_y: f64, size_z: f64, cell_size: f64) -> SimulationState {
        let cells_x: u64 = ((size_x / cell_size).ceil()) as u64;
        let cells_y: u64 = ((size_y / cell_size).ceil()) as u64;
        let cells_z: u64 = ((size_z / cell_size).ceil()) as u64;

        return SimulationState {
            cells_x,
            cells_y,
            cells_z,
            cell_size,
            energy: Vec::from_iter(iter::repeat_n(0.0, (cells_x * cells_y * cells_z) as usize)),
            density: Vec::from_iter(iter::repeat_n(0.0, (cells_x * cells_y * cells_z) as usize)),
            specific_heat: Vec::from_iter(iter::repeat_n(
                0.0,
                (cells_x * cells_y * cells_z) as usize,
            )),
            thermal_conductivity: Vec::from_iter(iter::repeat_n(
                0.0,
                (cells_x * cells_y * cells_z) as usize,
            )),
        };
    }

    /// Sets the material of all cells in a given volume
    pub fn set_material(&mut self, volume: &impl CellIterator, material: Material) {
        for (x, y, z) in volume.cell_iter(self.cell_size) {
            if let Some(index) = self.cell_to_index(x, y, z) {
                if let Some(d) = self.density.get_mut(index) {
                    *d = material.density;
                }
                if let Some(d) = self.specific_heat.get_mut(index) {
                    *d = material.specific_heat;
                }
                if let Some(d) = self.thermal_conductivity.get_mut(index) {
                    *d = material.thermal_conductivity;
                }
            }
        }
    }

    /// Sets the temperature of all cells in a given volume
    pub fn set_temperature(&mut self, volume: &impl CellIterator, temperature: f64) {
        for (x, y, z) in volume.cell_iter(self.cell_size) {
            if let Some(index) = self.cell_to_index(x, y, z) {
                if let (Some(e), Some(h), Some(d)) = (
                    self.energy.get_mut(index),
                    self.specific_heat.get(index),
                    self.density.get(index),
                ) {
                    let mass = d * self.cell_size.powi(3);
                    *e = temperature * mass * h;
                }
            }
        }
    }

    /// Adds energy to all cells in a given volume
    pub fn add_energy(&mut self, volume: &impl CellIterator, energy: f64) {
        for (x, y, z) in volume.cell_iter(self.cell_size) {
            if let Some(index) = self.cell_to_index(x, y, z) {
                if let Some(e) = self.energy.get_mut(index) {
                    *e += energy;
                }
            }
        }
    }

    /// Converts a cell x y z position to an index, returns None if cell is out of bounds
    fn cell_to_index(&self, x: u64, y: u64, z: u64) -> Option<usize> {
        if x >= self.cells_x {
            return None;
        }
        if y >= self.cells_y {
            return None;
        }
        if z >= self.cells_z {
            return None;
        }
        return Some((x + (y * self.cells_x) + (z * self.cells_x * self.cells_y)) as usize);
    }
}
