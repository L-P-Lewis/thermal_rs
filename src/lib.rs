#![deny(missing_docs)]
#![allow(unused)]
#![doc = include_str!("../README.md")]

/// Definition for simulation materials
pub mod material;
/// Definition of simulation runners
pub mod runner;
/// Definition of sim volumes and brushes
pub mod volume;

/// Thermal Simulation World utilities
///
/// Thermal Simulations opperate on a world, and a snapshot. A world represents the layout of
/// materials within the simulation reigon, and a snapshot represents the distribution of thermal
/// energy within the simulation reigon.
pub mod world;
