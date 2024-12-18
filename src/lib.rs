use pyo3::prelude::*;

const CHAR_OFFSET: u8 = 63;

#[inline(always)]
fn _encode_value(line: &mut String, mut value: i32) {
    let is_negative = value < 0;
    value <<= 1;
    if is_negative {
        value = !value;
    }
    loop {
        let mut chunk = (value & 0b11111) as u8;
        value >>= 5;
        let is_last_chunk = value == 0;
        if !is_last_chunk {
            chunk |= 0x20;
        }
        line.push(char::from(chunk + CHAR_OFFSET));
        if is_last_chunk {
            break;
        }
    }
}

#[inline(always)]
fn _encode(coordinates: Vec<Vec<f64>>, precision: i32, latlon: bool) -> String {
    let lat_idx = if latlon { 0 } else { 1 };
    let lon_idx = if latlon { 1 } else { 0 };
    let factor = 10_f64.powi(precision);
    let mut line = String::with_capacity(6 * 2 * coordinates.len());
    let mut last_lat = 0_i32;
    let mut last_lon = 0_i32;

    for coord in coordinates {
        let lat = (coord[lat_idx] * factor) as i32;
        _encode_value(&mut line, lat - last_lat);
        last_lat = lat;
        let lon = (coord[lon_idx] * factor) as i32;
        _encode_value(&mut line, lon - last_lon);
        last_lon = lon;
    }
    line
}

#[inline(always)]
fn _decode_value(mut value: i32) -> i32 {
    let is_negative = (value & 0x1) == 1;
    if is_negative {
        value = !value;
    }
    value >> 1
}

#[inline(always)]
fn _decode(line: &str, precision: i32, latlon: bool) -> Vec<(f64, f64)> {
    let factor = 10_f64.powi(precision);
    let mut coords = Vec::with_capacity(line.len() / 4);
    let mut last_lat = 0_i32;
    let mut last_lon = 0_i32;
    let mut first_set = false;
    let mut first_value = 0_i32;
    let mut second_value = 0_i32;
    let mut shift = 0;

    for c in line.chars() {
        let chunk = (c as u8) - CHAR_OFFSET;
        let is_last_chunk = (chunk & 0x20) == 0;
        let chunk = (chunk & 0b11111) as i32;
        if first_set {
            second_value |= chunk << shift;
        } else {
            first_value |= chunk << shift;
        }
        shift += 5;
        if is_last_chunk {
            if first_set {
                let lat = _decode_value(first_value) + last_lat;
                let lon = _decode_value(second_value) + last_lon;
                last_lat = lat;
                last_lon = lon;
                let lat = lat as f64 / factor;
                let lon = lon as f64 / factor;
                coords.push(if latlon { (lat, lon) } else { (lon, lat) });
                first_set = false;
                first_value = 0;
                second_value = 0;
                shift = 0;
            } else {
                first_set = true;
                shift = 0;
            }
        }
    }
    coords
}

#[pyfunction]
#[pyo3(signature = (coordinates, precision = 5))]
fn encode_lonlat(coordinates: Vec<Vec<f64>>, precision: i32) -> String {
    _encode(coordinates, precision, false)
}

#[pyfunction]
#[pyo3(signature = (coordinates, precision = 5))]
fn encode_latlon(coordinates: Vec<Vec<f64>>, precision: i32) -> String {
    _encode(coordinates, precision, true)
}

#[pyfunction]
#[pyo3(signature = (polyline, precision = 5))]
fn decode_lonlat(polyline: &str, precision: i32) -> Vec<(f64, f64)> {
    _decode(polyline, precision, false)
}

#[pyfunction]
#[pyo3(signature = (polyline, precision = 5))]
fn decode_latlon(polyline: &str, precision: i32) -> Vec<(f64, f64)> {
    _decode(polyline, precision, true)
}

#[pymodule(gil_used = false)]
#[pyo3(name = "_lib")]
fn lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_lonlat, m)?)?;
    m.add_function(wrap_pyfunction!(encode_latlon, m)?)?;
    m.add_function(wrap_pyfunction!(decode_lonlat, m)?)?;
    m.add_function(wrap_pyfunction!(decode_latlon, m)?)?;
    Ok(())
}
