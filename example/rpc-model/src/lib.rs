use pyo3::prelude::*;
pub mod model;


#[pymodule]
fn rpc_model(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<model::PyRpcAPIClient>()?;
    Ok(())
}
