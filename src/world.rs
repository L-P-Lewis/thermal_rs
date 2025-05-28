use base64::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    iter,
    path::Path,
    thread::current,
};

use crate::{material, material::Material, volume::CellIterator};

/// A builder for simulation worlds
///
/// Used to create a static simulation world.
///
/// ## Example
/// ```
/// # use thermal_rs::{world::SimWorldBuilder, material, volume::AABBVolume};
/// # use std::path::Path;
/// // Create a new world with a 1 meter cubic sim area
/// // half fill it with water, and build it with a 1cm voxel size
/// let new_world = SimWorldBuilder::new(1.0, 1.0, 1.0)
///     .with_material(material::WATER, Box::new(AABBVolume::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0)))
///     .build(0.01, 10, Path::new("new_world.yml"));
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
    pub fn build(
        self,
        resolution: f64,
        chunk_size: usize,
        filepath: &Path,
    ) -> Result<SimWorld, std::io::Error> {
        // Get world size
        let world_size_x = (self.x_size / resolution).ceil() as u64;
        let world_size_y = (self.y_size / resolution).ceil() as u64;
        let world_size_z = (self.z_size / resolution).ceil() as u64;

        // Initialize material map data
        let mut material_map: Vec<Material> = Vec::new();
        material_map.push(material::BLANK);
        let mut material_hash: HashMap<Material, u8> = HashMap::from([(material::BLANK, 0)]);

        // Fill in materials from brushes
        for (mat, _) in self.brush_opperations.iter() {
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
            if let None = found {
                material_map.push(mat.clone());
                material_hash.insert(*mat, (material_map.len() - 1) as u8);
            }
        }

        // Open new world file
        let mut file = File::create(filepath)?;

        // Write file header
        file.write(format!("x_size: {}\n", world_size_x).as_bytes());
        file.write(format!("y_size: {}\n", world_size_y).as_bytes());
        file.write(format!("z_size: {}\n", world_size_z).as_bytes());
        file.write(format!("cell_size: {}\n", resolution).as_bytes());
        file.write(format!("chunk_size: {}\n", chunk_size).as_bytes());
        file.write("materials: \n".as_bytes());

        // Write material section
        for mat in material_map.iter() {
            file.write("-\n".as_bytes());
            file.write(format!("  density: {}\n", mat.density).as_bytes());
            file.write(format!("  specific_heat: {}\n", mat.specific_heat).as_bytes());
            file.write(format!("  thermal_con_a: {}\n", mat.thermal_conductivity.0).as_bytes());
            file.write(format!("  thermal_con_b: {}\n", mat.thermal_conductivity.1).as_bytes());
            file.write(format!("  thermal_con_c: {}\n", mat.thermal_conductivity.2).as_bytes());
        }

        // Build and write chunks
        let chunk_count_x = (world_size_x as f64 / chunk_size as f64).ceil() as usize;
        let chunk_count_y = (world_size_y as f64 / chunk_size as f64).ceil() as usize;
        let chunk_count_z = (world_size_z as f64 / chunk_size as f64).ceil() as usize;

        file.write("chunks: \n".as_bytes());
        for chunk_x in 0..chunk_count_x {
            for chunk_y in 0..chunk_count_y {
                for chunk_z in 0..chunk_count_z {
                    let chunk_min_x: usize = chunk_x * chunk_size;
                    let chunk_max_x: usize = (chunk_min_x + chunk_size).min(world_size_x as usize);
                    let chunk_min_y: usize = chunk_y * chunk_size;
                    let chunk_max_y: usize = (chunk_min_y + chunk_size).min(world_size_y as usize);
                    let chunk_min_z: usize = chunk_z * chunk_size;
                    let chunk_max_z: usize = (chunk_min_z + chunk_size).min(world_size_z as usize);

                    let chunk_size_x = chunk_max_x - chunk_min_x;
                    let chunk_size_y = chunk_max_y - chunk_min_y;
                    let chunk_size_z = chunk_max_z - chunk_min_z;

                    let mut chunk_data: Vec<u8> = Vec::from_iter(iter::repeat_n(
                        0,
                        chunk_size_z * chunk_size_y * chunk_size_x,
                    ));

                    // Iterrate over all brushes
                    for (mat, brush) in self.brush_opperations.iter() {
                        let mat_id = material_hash
                            .get(mat)
                            .expect("Previously built map should have all materials.");
                        let (min_x, min_y, min_z) = brush.get_min(resolution);
                        let (max_x, max_y, max_z) = brush.get_max(resolution);

                        // Skip any brushes that do not overlap this chunk
                        let is_overlap = test_overlap(
                            chunk_min_x,
                            (min_x as usize),
                            chunk_max_x,
                            (max_x as usize),
                        ) && test_overlap(
                            chunk_min_y,
                            (min_y as usize),
                            chunk_max_y,
                            (max_y as usize),
                        ) && test_overlap(
                            chunk_min_z,
                            (min_z as usize),
                            chunk_max_z,
                            (max_z as usize),
                        );

                        if !is_overlap {
                            continue;
                        }

                        // Itterate over all positions in brush
                        for (x, y, z) in brush.cell_iter(resolution) {
                            if !(chunk_min_x <= x as usize && chunk_max_x >= x as usize) {
                                continue;
                            };
                            if !(chunk_min_y <= y as usize && chunk_max_y >= y as usize) {
                                continue;
                            };
                            if !(chunk_min_z <= z as usize && chunk_max_z >= z as usize) {
                                continue;
                            };

                            let sub_x = x as usize - chunk_min_x;
                            let sub_y = y as usize - chunk_min_y;
                            let sub_z = z as usize - chunk_min_z;

                            let ind = sub_x
                                + sub_y * (chunk_max_x - chunk_min_x)
                                + sub_z * (chunk_max_y - chunk_min_y) * (chunk_max_x - chunk_min_x);
                            if let Some(ch) = chunk_data.get_mut(ind) {
                                *ch = *mat_id;
                            }
                        }
                    }

                    // Write chunk data to file
                    file.write(
                        format!(
                            "- {}\n",
                            BASE64_STANDARD
                                .encode(encode_byte_stream(chunk_data.as_slice()).as_slice())
                        )
                        .as_bytes(),
                    );
                }
            }
        }

        return Ok(SimWorld {
            x_size: world_size_x as usize,
            y_size: world_size_y as usize,
            z_size: world_size_z as usize,
            cell_size: resolution,
            material_map,
            chunk_size,
            mat_file: file,
        });
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
///
/// Acts as a wrapper for a reference to a world file, which contains chunked cell material data
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
    // The edge length of a chunk, in cells
    chunk_size: usize,
    // The handle for the chunk data file
    mat_file: File,
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

// Internal functions
fn test_overlap(mina: usize, minb: usize, maxa: usize, maxb: usize) -> bool {
    mina <= maxb && minb <= maxa
}

fn encode_byte_stream<'a>(bytes: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    let mut current: Option<u8> = None;
    let mut run: u8 = 0;

    for byte in bytes.iter() {
        if current == None {
            current = Some(*byte);
        }

        if let Some(cur_byte) = current {
            if cur_byte == *byte {
                run += 1;
            } else {
                out.push(run);
                out.push(cur_byte);
                current = None;
                run = 0;
            }
            if run == u8::MAX {
                out.push(run);
                out.push(cur_byte);
                current = None;
                run = 0;
            }
        }
    }
    if let Some(cur_byte) = current {
        out.push(run);
        out.push(cur_byte);
    }

    return out;
}
