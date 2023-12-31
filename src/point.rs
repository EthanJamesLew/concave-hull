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
}

/// Point Value -- Neighbor Information
/// Point value captures a point, with a distance and angle quantity with
/// respect to another point
pub struct PointValue {
    pub point: Point,
    pub distance: f64,
    pub angle: f64,
}
