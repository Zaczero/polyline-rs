use geo_types::{Coord, LineString};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
#[pyo3(signature = (coordinates, precision = 5))]
fn encode_lonlat(coordinates: Vec<Vec<f64>>, precision: u32) -> PyResult<String> {
    let line = LineString(
        coordinates
            .into_iter()
            .map(|c| Coord { x: c[0], y: c[1] })
            .collect(),
    );
    match polyline::encode_coordinates(line, precision) {
        Ok(polyline) => Ok(polyline),
        Err(err) => Err(PyValueError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (coordinates, precision = 5))]
fn encode_latlon(coordinates: Vec<Vec<f64>>, precision: u32) -> PyResult<String> {
    let line = LineString(
        coordinates
            .into_iter()
            .map(|c| Coord { x: c[1], y: c[0] })
            .collect(),
    );
    match polyline::encode_coordinates(line, precision) {
        Ok(polyline) => Ok(polyline),
        Err(err) => Err(PyValueError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (polyline, precision = 5))]
fn decode_lonlat(polyline: &str, precision: u32) -> PyResult<Vec<(f64, f64)>> {
    let line = match polyline::decode_polyline(polyline, precision) {
        Ok(line) => line,
        Err(err) => return Err(PyValueError::new_err(err.to_string())),
    };
    Ok(line.0.into_iter().map(|c| (c.x, c.y)).collect())
}

#[pyfunction]
#[pyo3(signature = (polyline, precision = 5))]
fn decode_latlon(polyline: &str, precision: u32) -> PyResult<Vec<(f64, f64)>> {
    let line = match polyline::decode_polyline(polyline, precision) {
        Ok(line) => line,
        Err(err) => return Err(PyValueError::new_err(err.to_string())),
    };
    Ok(line.0.into_iter().map(|c| (c.y, c.x)).collect())
}

#[pymodule]
#[pyo3(name = "_lib")]
fn lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_lonlat, m)?)?;
    m.add_function(wrap_pyfunction!(encode_latlon, m)?)?;
    m.add_function(wrap_pyfunction!(decode_lonlat, m)?)?;
    m.add_function(wrap_pyfunction!(decode_latlon, m)?)?;
    Ok(())
}
