use std::f64::consts::PI;

/// Point Primitives
use pyo3::prelude::*;

/// 2D Point with Identifier
/// The identified is used to identify points between data structures
/// (the points list and the kd-tree)
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// x coordinate
    #[pyo3(get, set)]
    pub x: f64,
    /// y coordinate
    #[pyo3(get, set)]
    pub y: f64,
    /// identifier
    #[pyo3(get, set)]
    pub id: u64,
}

#[pymethods]
impl Point {
    /// constructor for python bindings
    #[new]
    fn new(x: f64, y: f64, id: u64) -> Self {
        Point { x, y, id }
    }

    pub fn angle(&self, b: &Point) -> f64 {
        let angle = -((b.y - self.y).atan2(b.x - self.x));
        normalise_angle(angle)
    }
}

/// Point Value -- Neighbor Information
/// Point value captures a point, with a distance and angle quantity with
/// respect to another point
pub struct PointValue {
    /// identified point
    pub point: Point,
    /// distance to other
    pub distance: f64,
    /// angle from other
    pub angle: f64,
}

pub fn normalise_angle(radians: f64) -> f64 {
    if radians < 0.0 {
        radians + PI + PI
    } else {
        radians
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    fn test_angle() {
        let test = |p: Point, expected: f64| {
            let actual = to_degrees(
                Point {
                    x: 0.0,
                    y: 0.0,
                    id: 0,
                }
                    .angle(&p),
            );
            assert!(
                (actual == expected),
                "Test failed for point: ({}, {})",
                p.x,
                p.y
            );
        };

        let value = to_degrees((3.0f64 / 4.0).atan());

        test(
            Point {
                x: 5.0,
                y: 0.0,
                id: 0,
            },
            0.0,
        );
        test(
            Point {
                x: 4.0,
                y: 3.0,
                id: 0,
            },
            360.0 - value,
        );
        test(
            Point {
                x: 3.0,
                y: 4.0,
                id: 0,
            },
            270.0 + value,
        );
        test(
            Point {
                x: 0.0,
                y: 5.0,
                id: 0,
            },
            270.0,
        );
        test(
            Point {
                x: -3.0,
                y: 4.0,
                id: 0,
            },
            270.0 - value,
        );
        test(
            Point {
                x: -4.0,
                y: 3.0,
                id: 0,
            },
            180.0 + value,
        );
        test(
            Point {
                x: -5.0,
                y: 0.0,
                id: 0,
            },
            180.0,
        );
        test(
            Point {
                x: -4.0,
                y: -3.0,
                id: 0,
            },
            180.0 - value,
        );
        test(
            Point {
                x: -3.0,
                y: -4.0,
                id: 0,
            },
            90.0 + value,
        );
        test(
            Point {
                x: 0.0,
                y: -5.0,
                id: 0,
            },
            90.0,
        );
        test(
            Point {
                x: 3.0,
                y: -4.0,
                id: 0,
            },
            90.0 - value,
        );
        test(
            Point {
                x: 4.0,
                y: -3.0,
                id: 0,
            },
            value,
        );
    }

    #[test]
    fn test_angle_function() {
        test_angle();
    }
}
