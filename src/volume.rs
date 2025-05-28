/// Trait for structs which represent a volume of world space, returns an iterator over tupes of
/// (x, y, z) positions of cells within the volume
pub trait CellIterator {
    /// Create an iterator over all positions within a given volume with a given cell size
    fn cell_iter(self: &Self, cell_size: f64) -> Box<dyn Iterator<Item = (u64, u64, u64)>>;

    /// Get the minimum position of the volume
    fn get_min(self: &Self, cell_size: f64) -> (u64, u64, u64);

    /// Get the maximum position of the volume
    fn get_max(self: &Self, cell_size: f64) -> (u64, u64, u64);
}

/// Struct for representing an axis aligned volume
#[derive(Debug, Clone)]
pub struct AABBVolume {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

impl AABBVolume {
    /// Create a new AABB Volume
    pub fn new(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> AABBVolume {
        AABBVolume {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }
}

impl CellIterator for AABBVolume {
    fn cell_iter(self: &Self, cell_size: f64) -> Box<dyn Iterator<Item = (u64, u64, u64)>> {
        return Box::new(AABBVolumeIter {
            min_x: (self.min_x / cell_size).floor() as u64,
            min_y: (self.min_y / cell_size).floor() as u64,
            min_z: (self.min_z / cell_size).floor() as u64,
            len_x: ((self.max_x - self.min_x) / cell_size).ceil() as u64,
            len_y: ((self.max_y - self.min_y) / cell_size).ceil() as u64,
            len_z: ((self.max_z - self.min_z) / cell_size).ceil() as u64,
            x: 0,
            y: 0,
            z: 0,
        });
    }

    fn get_min(self: &Self, cell_size: f64) -> (u64, u64, u64) {
        (
            (self.min_x / cell_size).floor() as u64,
            (self.min_y / cell_size).floor() as u64,
            (self.min_z / cell_size).floor() as u64,
        )
    }

    fn get_max(self: &Self, cell_size: f64) -> (u64, u64, u64) {
        (
            (self.max_x / cell_size).ceil() as u64,
            (self.max_y / cell_size).ceil() as u64,
            (self.max_z / cell_size).ceil() as u64,
        )
    }
}

struct AABBVolumeIter {
    min_x: u64,
    min_y: u64,
    min_z: u64,
    len_x: u64,
    len_y: u64,
    len_z: u64,
    x: u64,
    y: u64,
    z: u64,
}

impl Iterator for AABBVolumeIter {
    type Item = (u64, u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.z > self.len_z {
            return None;
        }

        self.x += 1;
        if self.x > self.len_x {
            self.y += 1;
            self.x = 0;
        }

        if self.y > self.len_y {
            self.z += 1;
            self.y = 0;
        }

        return Some((
            self.min_x + self.x,
            self.min_y + self.y,
            self.min_z + self.z,
        ));
    }
}
