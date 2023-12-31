use crate::point::Point;

use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

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
        points.push(Point {
            x,
            y,
            id: (i as u64),
        });
    }

    Ok(points)
}

#[pyfunction]
pub fn concave_hull_2d(
    py: Python<'_>,
    dataset: &PyArray2<f64>,
    k: usize,
    iterate: bool,
) -> PyResult<Py<PyArray2<f64>>> {
    let mut dataset_vec = numpy_to_vec_points(dataset.readonly())?;
    let result = crate::concave_hull(&mut dataset_vec, k, iterate);

    // Create a new 2D NumPy array
    let mut array = unsafe { PyArray2::<f64>::new(py, [result.len(), 3], false) };

    // Obtain a mutable slice of the entire array
    let array_slice = unsafe { array.as_slice_mut().unwrap() };

    // Fill the array with data from the Vec<Point>
    for (i, point) in result.iter().enumerate() {
        let start_idx = i * 3;
        array_slice[start_idx] = point.x;
        array_slice[start_idx + 1] = point.y;
        array_slice[start_idx + 2] = point.id as f64; // Assuming you want to store the ID as a float
    }

    Ok(array.into_py(py))
}

#[pymodule]
pub fn concave_hull(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_function(wrap_pyfunction!(concave_hull_2d, m)?)?;
    Ok(())
}
