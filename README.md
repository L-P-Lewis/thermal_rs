thermal_rs provides utlities for preforming discreet simulations of heat conduction in voxelized worlds.

## Features

- Creation of simulation worlds from brush based geometry and 3d models
    - Worlds and States are stored on disc and streamed in live when needed
- Provides multiple implementations of heat flow simulation
    - Single Threaded CPU simulation
    - Multithreaded CPU simulation
    - GPU simulation

## Usage

```rust
use thermal_rs::{world::{SimWorld, SimWorldBuilder, SimState}, material, material::Material, volume::AABBVolume, runner::{SimRunner, cpu::CPUSimRunner}};
use std::path::Path;

fn main() {
    // Create a simulation world with a base of water and a resolution of 0.01 voxel/m
    let sim_world = SimWorldBuilder::new(5.0, 2.0, 5.0)
        .with_material(
            material::WATER,
            Box::new(AABBVolume::new(0.0, 0.0, 0.0, 2.0, 1.0, 1.0))
        ).build(0.01, 10, Path::new("./sim_world.yaml")).unwrap();

    let mut initial_state = sim_world.get_blank_sim_state();
    initial_state = sim_world.set_sim_state_temperature(
        initial_state,
        300.0,
        &AABBVolume::new(0.0, 0.0, 0.0, 5.0, 1.0, 5.0)
    ).unwrap();

    // Create a simple cpu simulation runner
    let cpu_runner = CPUSimRunner {};

    // Calculate the simulation at an advanced state of 1 second into the future given a timestep of 0.01 seconds
        cpu_runner.advance_simulation(
            &sim_world,
            &initial_state,
            1.0,
            0.1
        );
}

```

## File Formats

Due to the size of the simulation thermal_rs is optimized to work with, data is stored on disc and streamed in dynamically when needed for simulation. These take the form of YAML files with some number of plaintext header fields, followed by Base64 encoded chunked world data, stored in X>Y>Z order.

### World Files

#### Header Fields
| Field | Data |
|---|---|
| x_size | An integer representing the x size of the world in voxels | 
| y_size | An integer representing the y size of the world in voxels | 
| z_size | An integer representing the z size of the world in voxels | 
| cell_size | The side length of a voxel in meters |
| chunk_size | The side length of a chunk in voxels |
| materials | A sequence of Material mappings representing a material in the world, this map is limited to a length of 256 |
| chunks | A sequence of Chunk data blocks |

#### Material Fields
| Field | Data |
|---|---|
| density | The density of the material in Kg/m^3 |
| specific_heat | The specific heat of the material |
| thermal_con_a | The "a" coefficient of the thermal conductivity polynomial |
| thermal_con_b | The "b" coefficient of the thermal conductivity polynomial |
| thermal_con_c | The "c" coefficient of the thermal conductivity polynomial |

#### Chunk Data Blocks
Chunk data blocks are stored as Run-Length Encoded data, encoded in base 64. The number of voxels encoded is dynamically determined based on the placing of the chunk within the world. Voxels are encoded in X>Y>Z order. The materials are encoded as an 8 bit run length, and an 8 bit index into the materials map.

