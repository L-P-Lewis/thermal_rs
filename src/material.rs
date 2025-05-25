#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    /// The density of the material in kg/m^3
    pub density: f64,
    /// The specific heat of the material in J / K * kg
    pub specific_heat: f64,
    /// The thermal conductivity of the material in (J / m^2 * s) / K
    pub thermal_conductivity: f64,
}

impl Material {
    pub fn new(density: f64, specific_heat: f64, thermal_conductivity: f64) -> Material {
        return Material {
            density,
            specific_heat,
            thermal_conductivity,
        };
    }

    pub fn blank() -> Material {
        Material::new(1.0, 1.0, 0.0)
    }
}
