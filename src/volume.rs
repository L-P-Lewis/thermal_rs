/// Trait for structs which represent a volume of world space, returns an iterator over tupes of
/// (x, y, z) positions of cells within the volume
pub trait CellIterator {
    /// Create an iterator over all positions within a given volume with a given cell size
    fn cell_iter(self: &Self, cell_size: f32) -> Box<dyn Iterator<Item = (usize, usize, usize)>>;
}

/// Struct for representing an axis aligned volume
#[derive(Debug, Clone)]
pub struct AABBVolume {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
}

impl AABBVolume {
    /// Create a new AABB Volume
    pub fn new(
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
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
    fn cell_iter(self: &Self, cell_size: f32) -> Box<dyn Iterator<Item = (usize, usize, usize)>> {
        return Box::new(AABBVolumeIter {
            min_x: self.min_x.floor() as usize,
            min_y: self.min_y.floor() as usize,
            min_z: self.min_z.floor() as usize,
            len_x: ((self.max_x - self.min_x) / cell_size).ceil() as usize,
            len_y: ((self.max_y - self.min_y) / cell_size).ceil() as usize,
            len_z: ((self.max_z - self.min_z) / cell_size).ceil() as usize,
            x: 0,
            y: 0,
            z: 0,
        });
    }
}

struct AABBVolumeIter {
    min_x: usize,
    min_y: usize,
    min_z: usize,
    len_x: usize,
    len_y: usize,
    len_z: usize,
    x: usize,
    y: usize,
    z: usize,
}

impl Iterator for AABBVolumeIter {
    type Item = (usize, usize, usize);
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
