use crate::core;
use std::sync::LazyLock;

static LUT: LazyLock<[f32; 0x2000]> = LazyLock::new(core::create_lut);

/// A compressed unit vector representation with a size of two bytes.
pub struct CompUnitVec {
    value: u16,
}

impl CompUnitVec {
    /// Constructor to create a new compressed unit vector from three floating point values.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: core::pack(x, y, z),
        }
    }

    /// Constructor to create a new compressed unit vector an already compressed u16 value.
    pub fn from_u16(value: u16) -> Self {
        Self { value }
    }

    /// Constructor to create a new compressed unit vector from a slice of three floating point values.
    pub fn from_slice(v: &[f32; 3]) -> Self {
        Self {
            value: core::pack(v[0], v[1], v[2]),
        }
    }

    /// Unpack compressed unit vector to retrieve an array of the three floating point components.
    pub fn get(&self) -> [f32; 3] {
        core::unpack(self.value, &LUT)
    }

    /// Get the value of the compressed u16 representation of the unit vector.
    pub fn get_u16(&self) -> u16 {
        self.value
    }
}
