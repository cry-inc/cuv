use cuv::*;
use rand::{Rng, SeedableRng};

#[test]
fn new() {
    let cv = CompUnitVec::new(1.0, 0.0, 0.0);
    assert_eq!(cv.get(), [1.0, 0.0, 0.0]);
}

#[test]
fn from_vec() {
    let ov: [f32; 3] = [1.0, 0.0, 0.0];
    let cv = CompUnitVec::from_slice(&ov);
    assert_eq!(cv.get(), ov);
}

#[test]
fn from_u16() {
    let cv = CompUnitVec::from_u16(255);
    assert_eq!(cv.get(), [1.0, 0.0, 0.0]);
}

#[test]
fn get_u16() {
    let cv = CompUnitVec::new(1.0, 0.0, 0.0);
    assert_eq!(cv.get_u16(), 255);
}

#[test]
fn test_all_for_validitiy() {
    for u in 0..u16::MAX {
        let cv = CompUnitVec::from_u16(u);
        let [x, y, z] = cv.get();

        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
        assert!(z >= -1.0 && z <= 1.0);

        let len = (x * x + y * y + z * z).sqrt();
        let diff_one = (1.0 - len).abs();
        assert!(diff_one < 0.000001);
    }
}

#[test]
fn random_vectors() {
    const MAX_ALLOWED_ERR: f32 = 0.03;
    let mut rng = rand::rngs::StdRng::from_seed([0_u8; 32]);
    for _ in 0..10000000 {
        let x = rng.gen::<f32>() - 0.5;
        let y = rng.gen::<f32>() - 0.5;
        let z = rng.gen::<f32>() - 0.5;
        let cv = CompUnitVec::new(x, y, z);

        let len = (x * x + y * y + z * z).sqrt();
        let nx = x / len;
        let ny = y / len;
        let nz = z / len;

        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        assert!((len - 1.0).abs() < 0.000001);

        let [cx, cy, cz] = cv.get();
        let dx = (cx - nx).abs();
        let dy = (cy - ny).abs();
        let dz = (cz - nz).abs();

        assert!(dx < MAX_ALLOWED_ERR);
        assert!(dy < MAX_ALLOWED_ERR);
        assert!(dz < MAX_ALLOWED_ERR);
    }
}

#[test]
fn low_level_interface() {
    let cv = pack(1.0, 0.0, 0.0);
    assert_eq!(cv, 255);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [1.0, 0.0, 0.0]);
}

#[test]
fn all_zeros() {
    let cv = pack(0.0, 0.0, 0.0);
    assert_eq!(cv, 0);
}

#[test]
fn one_infinity() {
    let cv = pack(f32::INFINITY, 0.0, 0.0);
    assert_eq!(cv, 0);
}

#[test]
fn one_neg_infinity() {
    let cv = pack(f32::NEG_INFINITY, 0.0, 0.0);
    assert_eq!(cv, 32768);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [-0.0, 0.0, 1.0]);
}

#[test]
fn all_infinity() {
    let cv = pack(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    assert_eq!(cv, 0);
}

#[test]
fn all_neg_infinity() {
    let cv = pack(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    assert_eq!(cv, 57344);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [-0.0, -0.0, -1.0]);
}

#[test]
fn one_nan() {
    let cv = pack(f32::NAN, 0.0, 0.0);
    assert_eq!(cv, 0);
}

#[test]
fn all_nan() {
    let cv = pack(f32::NAN, f32::NAN, f32::NAN);
    assert_eq!(cv, 0);
}

#[test]
fn min_floats() {
    let cv = pack(f32::MIN, f32::MIN, f32::MIN);
    assert_eq!(cv, 57344);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [-0.0, -0.0, -1.0]);
}

#[test]
fn max_floats() {
    let cv = pack(f32::MAX, f32::MAX, f32::MAX);
    assert_eq!(cv, 0);
}

#[test]
fn one_min_pos() {
    let cv = pack(f32::MIN_POSITIVE, 0.0, 0.0);
    assert_eq!(cv, 16511);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [0.999969, -0.0, -0.007873772]);
}

#[test]
fn all_min_pos() {
    let cv = pack(f32::MIN_POSITIVE, f32::MIN_POSITIVE, f32::MIN_POSITIVE);
    assert_eq!(cv, 16512);

    let lut = create_lut();
    let xyz = unpack(cv, &lut);
    assert_eq!(xyz, [0.007999744, -0.0, 0.99996805]);
}
