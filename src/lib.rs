use pyo3::prelude::*;
use geohash::{encode, decode, Coord};

#[pyfunction]
fn decode_geohash(hash_str: &str) -> PyResult<(f64, f64)> {
    let (c, _, _) = decode(hash_str).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    Ok((c.x, c.y))  // Assuming Coord<f64> has x and y fields
}

#[pyfunction]
fn decode_geohash_with_offset(hash_str: &str) -> PyResult<(f64, f64, f64, f64)> {
    let (c, lon_offset, lat_offset) = decode(hash_str).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    Ok((c.x, c.y, lat_offset, lon_offset))  // Assuming Coord<f64> has x and y fields
}


#[pyfunction]
fn encode_geohash(lat: f64, lon: f64, precision: usize) -> PyResult<String> {
    let c = Coord{x: lat, y: lon};

    let geohash = encode(c, precision).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

    Ok(geohash)

}

#[pymodule]
fn geohash_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_geohash, m)?)?;
    m.add_function(wrap_pyfunction!(decode_geohash_with_offset, m)?)?;
    m.add_function(wrap_pyfunction!(encode_geohash, m)?)?;
    Ok(())
}
