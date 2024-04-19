use hypermath::vector::Vector2D;
use rug::Float;

const F64_PRECISION: u32 = 53;

#[test]
fn new_eq_from_f64() {
    assert_eq!(
        Vector2D::new(Float::with_val(F64_PRECISION, 2.4), Float::with_val(F64_PRECISION, -7.4)),
        Vector2D::from_f64(2.4, -7.4),
    )
}

#[test]
fn new_eq_from_vec() {
    assert_eq!(
        Vector2D::new(Float::with_val(F64_PRECISION, 2.4), Float::with_val(F64_PRECISION, -7.4)),
        Vector2D::from_vec(vec![2.4, -7.4]).unwrap(),
    )
}

#[test]
fn fail_on_too_little_values() {
    assert!(Vector2D::from_vec(vec![1.0]).is_err())
}