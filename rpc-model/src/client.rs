use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use std::net::SocketAddr;
use tarpc::{client, serde_transport::tcp};
use tokio_serde::formats::Bincode;

use crate::model::RpcAPIClient;

#[pyclass]
pub struct PyRpcClient {
    client: RpcAPIClient,
}

#[pymethods]
impl PyRpcClient {
    #[staticmethod]
    fn connect<'p>(py: Python<'p>, address: String) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let addr: SocketAddr = address.parse().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid address: {}", e))
            })?;

            let transport = tcp::connect(addr, Bincode::default).await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyConnectionError, _>(format!(
                    "Failed to connect: {}",
                    e
                ))
            })?;

            let client = RpcAPIClient::new(client::Config::default(), transport).spawn();

            Ok(PyRpcClient { client })
        })
    }

    fn hello<'p>(&self, py: Python<'p>, name: String) -> PyResult<Bound<'p, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client.hello(tarpc::context::current(), name).await;
            match response {
                Ok(response) => Ok(response),
                Err(e) => {
                    // Convert error to string without using py (since py is not Send)
                    let error_msg = format!("{:?}", e);
                    Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error_msg))
                }
            }
        })
    }
}
