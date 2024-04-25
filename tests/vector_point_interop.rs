use hypermath::{point::Point2D, vector::Vector2D};

#[test]
/// Assert that the vector created from a point equals the expected vector
fn create_vector_from_point() {
    assert_eq!(
        Vector2D::from_f64(3.2, -5.5),
        Point2D::from_f64(0.0, 0.0).create_vector2d_self(Point2D::from_f64(3.2, -5.5))
    )
}

