use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub id: u64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64, id: u64) -> Self {
        Point { x, y, id }
    }
}

pub struct PointValue {
    pub point: Point,
    pub distance: f64,
    pub angle: f64,
}
