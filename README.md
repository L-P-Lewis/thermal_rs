thermal_rs provides utlities for preforming discreet simulations of heat conduction in voxelized worlds.

## Features

- Creation of simulation worlds from brush based geometry and 3d models
- Provides multiple implementations of heat flow simulation
    - Single Threaded CPU simulation
    - Multithreaded CPU simulation
    - GPU simulation

## Usage

```rust
use thermal_rs::{world::{SimWorld, SimWorldBuilder, SimState}, material::Material, volume::AABBVolume, runner::{SimRunner, cpu::CPUSimRunner}};

fn main() {
    // Create a simulation world with a base of water and a resolution of 1 voxel/cm
    let sim_world = SimWorldBuilder::new(10.0, 10.0, 10.0)
        .with_materials(
            Material {density: 10.0, specific_heat: 4187.0, thermal_conductivity:7000.0},
            &AABBVolume::new(0.0, 0.0, 0.0, 10.0, 5.0, 10.0)
        ).build(0.001);

    let mut initial_state = sim_world.get_blank_sim_state();
    initial_state = sim_world.set_sim_state_temperature(
        initial_state,
        300.0,
        &AABBVolume::new(0.0, 0.0, 0.0, 5.0, 5.0, 5.0)
    ).unwrap();

    // Create a simple cpu simulation runner
    let cpu_runner = CPUSimRunner {};

    // Calculate the simulation at an advanced state of 1 second into the future given a timestep of 0.01 seconds
        cpu_runner.advance_simulation(
            &sim_world,
            &initial_state,
            1.0,
            0.01
        );

    
}


```
