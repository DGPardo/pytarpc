use pyo3::prelude::*;
pub mod client;
pub mod model;

use client::*;

#[pymodule]
fn rpc_model(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRpcClient>()?;
    Ok(())
}
