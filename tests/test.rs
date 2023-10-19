use cuv::CompUnitVec;
use rand::{Rng, SeedableRng};

#[test]
fn new() {
    let cv = CompUnitVec::new(1.0, 0.0, 0.0);
    let v = cv.get();
    assert_eq!([1.0, 0.0, 0.0], v);
}

#[test]
fn from_vec() {
    let ov: [f32; 3] = [1.0, 0.0, 0.0];
    let cv = CompUnitVec::from_slice(&ov);
    let v = cv.get();
    assert_eq!(ov, v);
}

#[test]
fn from_u16() {
    let cv = CompUnitVec::from_u16(255);
    let v = cv.get();
    assert_eq!([1.0, 0.0, 0.0], v);
}

#[test]
fn get_u16() {
    let cv = CompUnitVec::new(1.0, 0.0, 0.0);
    let u16 = cv.get_u16();
    assert_eq!(255, u16);
}

#[test]
fn test_all_for_validitiy() {
    for u in 0..u16::MAX {
        let cv = CompUnitVec::from_u16(u);
        let xyz = cv.get();

        assert!(xyz[0] <= 1.0);
        assert!(xyz[1] <= 1.0);
        assert!(xyz[2] <= 1.0);

        let len = (xyz[0] * xyz[0] + xyz[1] * xyz[1] + xyz[2] * xyz[2]).sqrt();
        let diff_one = (1.0 - len).abs();
        assert!(diff_one < 0.000001);
    }
}

#[test]
fn random_vectors() {
    const MAX_ALLOWED_ERR: f32 = 0.03;
    let mut rng = rand::rngs::StdRng::from_seed([0_u8; 32]);
    for _ in 0..10000000 {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        let z: f32 = rng.gen();

        let len = (x * x + y * y + z * z).sqrt();
        let nx = x / len;
        let ny = y / len;
        let nz = z / len;
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        assert!((len - 1.0).abs() < 0.000001);

        let cv = CompUnitVec::new(x, y, z);
        let xyz = cv.get();
        let cx = xyz[0];
        let cy = xyz[1];
        let cz = xyz[2];

        let dx = (cx - nx).abs();
        let dy = (cy - ny).abs();
        let dz = (cz - nz).abs();

        assert!(dx < MAX_ALLOWED_ERR);
        assert!(dy < MAX_ALLOWED_ERR);
        assert!(dz < MAX_ALLOWED_ERR);
    }
}
