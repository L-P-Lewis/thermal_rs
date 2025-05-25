pub mod material;
pub mod simulate;

#[derive(Debug, Clone)]
pub struct Volume {
    min_x : f64
    min_y : f64
    min_z : f64
    max_x : f64
    max_y : f64
    max_z : f64
}

impl Volume {
    pub fn new(
    min_x : f64,
    min_y : f64,
    min_z : f64,
    max_x : f64,
    max_y : f64,
    max_z : f64,) -> Volume {
        Volume {min_x,
    min_y,
    min_z,
    max_x,
    max_y,
    max_z}
    }
}
