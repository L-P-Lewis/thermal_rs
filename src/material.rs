/// Default material aproximating the properties of water at sea level atmospheric pressure
static WATER: Material = Material {
    density: 1000.0,
    specific_heat: 4000.0,
    thermal_conductivity: (-0.000006454, 0.005208, -0.3686),
};

/// Represents a material type
#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    /// The density of the material in kg/m^3
    pub density: f64,
    /// The specific heat of the material in J / K * kg
    pub specific_heat: f64,
    /// Coefficients (a, b, c) for an equation for thermal conductivity C of the material in W / m K. Given as C = a*k^2 + b*c + c
    pub thermal_conductivity: (f64, f64, f64),
}

impl Material {
    /// Get the "blank" material type, a perfect insulator
    pub fn blank() -> Material {
        Material {
            density: 1.0,
            specific_heat: 1.0,
            thermal_conductivity: (0.0, 0.0, 0.0),
        }
    }
}
