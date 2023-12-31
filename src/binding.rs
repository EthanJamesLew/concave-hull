use crate::point::{Point};

use pyo3::prelude::*;
use numpy::{PyArray2, PyReadonlyArray2};

fn numpy_to_vec_points(array: PyReadonlyArray2<f64>) -> PyResult<Vec<Point>> {
    let rows = array.shape()[0];
    let columns = array.shape()[1];

    // Check that the NumPy array has the correct number of columns (3 in this case: x, y, id)
    if columns != 2 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Array must have 2 columns",
        ));
    }

    let mut points = Vec::with_capacity(rows);

    for i in 0..rows {
        let x = *array.get([i, 0]).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyIndexError, _>("Index out of bounds")
        })?;
        let y = *array.get([i, 1]).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyIndexError, _>("Index out of bounds")
        })?;
        points.push(Point { x, y, id: (i as u64) });
    }

    Ok(points)
}

#[pyfunction]
pub fn concave_hull_py(dataset: &PyArray2<f64>, k: usize, iterate: bool) -> PyResult<Vec<Point>> {
    let mut dataset_vec = numpy_to_vec_points(dataset.readonly())?;
    let result = crate::concave_hull(&mut dataset_vec, k, iterate);
    Ok(result)
}

#[pymodule]
pub fn concave_hull(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_function(wrap_pyfunction!(concave_hull_py, m)?)?;
    Ok(())
}
