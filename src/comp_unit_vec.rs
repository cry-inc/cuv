use crate::core;

static LUT: once_cell::sync::Lazy<[f32; 0x2000]> = once_cell::sync::Lazy::new(core::create_lut);

pub struct CompUnitVec {
    value: u16,
}

impl CompUnitVec {
    pub fn from_u16(value: u16) -> Self {
        Self { value }
    }

    pub fn from_slice(v: &[f32; 3]) -> Self {
        Self {
            value: core::pack(v[0], v[1], v[2]),
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: core::pack(x, y, z),
        }
    }

    pub fn get(&self) -> [f32; 3] {
        core::unpack(self.value, &LUT)
    }

    pub fn get_u16(&self) -> u16 {
        self.value
    }
}
