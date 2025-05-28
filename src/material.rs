#[derive(Debug, PartialEq, Clone)]

/// Represents a material type
pub struct Material {
    /// The density of the material in kg/m^3
    pub density: f64,
    /// The specific heat of the material in J / K * kg
    pub specific_heat: f64,
    /// The thermal conductivity of the material in (J / m^2 * s) / K
    pub thermal_conductivity: f64,
}

impl Material {
    /// Get the "blank" material type, a perfect insulator
    pub fn blank() -> Material {
        Material {
            density: 1.0,
            specific_heat: 1.0,
            thermal_conductivity: 0.0,
        }
    }
}
