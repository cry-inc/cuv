const SIGN_MASK: u16 = 0xe000;
const XSIGN_MASK: u16 = 0x8000;
const YSIGN_MASK: u16 = 0x4000;
const ZSIGN_MASK: u16 = 0x2000;
const TOP_MASK: u16 = 0x1f80;
const BOTTOM_MASK: u16 = 0x007f;

pub fn create_lut() -> [f32; 0x2000] {
    let mut table = [0_f32; 0x2000];
    for idx in 0..0x2000 {
        let mut xbits: i32 = idx >> 7;
        let mut ybits: i32 = idx & BOTTOM_MASK as i32;

        if (xbits + ybits) >= 127 {
            xbits = 127 - xbits;
            ybits = 127 - ybits;
        }

        let x = xbits as f32;
        let y = ybits as f32;
        let z = (126 - xbits - ybits) as f32;

        table[idx as usize] = 1.0_f32 / (y * y + z * z + x * x).sqrt();
        assert!(table[idx as usize].is_finite());
    }
    table
}

pub fn pack(mut x: f32, mut y: f32, mut z: f32) -> u16 {
    let mut res: u16 = 0;
    if x < 0.0 {
        res |= XSIGN_MASK;
        x = -x;
    }
    if y < 0.0 {
        res |= YSIGN_MASK;
        y = -y;
    }
    if z < 0.0 {
        res |= ZSIGN_MASK;
        z = -z;
    }

    let w = 126.0 / (x + y + z);
    let mut xbits = (x * w) as i32;
    let mut ybits = (y * w) as i32;

    //assert!(xbits < 127);
    //assert!(xbits >= 0);
    //assert!(ybits < 127);
    //assert!(ybits >= 0);

    if xbits >= 64 {
        xbits = 127 - xbits;
        ybits = 127 - ybits;
    }

    res |= (xbits << 7) as u16;
    res |= ybits as u16;

    res
}

pub fn unpack(cv: u16, lut: &[f32; 0x2000]) -> [f32; 3] {
    let mut xbits = ((cv & TOP_MASK) >> 7) as i32;
    let mut ybits = (cv & BOTTOM_MASK) as i32;

    if (xbits + ybits) >= 127 {
        xbits = 127 - xbits;
        ybits = 127 - ybits;
    }

    let index = cv & !SIGN_MASK;
    let uvadj = lut[index as usize];
    let mut x = uvadj * xbits as f32;
    let mut y = uvadj * ybits as f32;
    let mut z = uvadj * (126 - xbits - ybits) as f32;

    if cv & XSIGN_MASK != 0 {
        x = -x;
    }
    if cv & YSIGN_MASK != 0 {
        y = -y;
    }
    if cv & ZSIGN_MASK != 0 {
        z = -z;
    }

    [x, y, z]
}
