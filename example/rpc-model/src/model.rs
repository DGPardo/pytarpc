use pyo3::prelude::*;
use pytarpc::tarpc_python_client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass]
pub struct CustomDataType {
    field1: String,
    field2: Vec<f32>,
}

#[pymethods]
impl CustomDataType {
    #[new]
    fn new(field1: String, field2: Vec<f32>) -> Self {
        CustomDataType { field1, field2 }
    }

    fn __repr__(&self) -> String {
        format!(
            "CustomDataType(field1={}, field2={:?})",
            self.field1, self.field2
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[tarpc_python_client]
#[tarpc::service]
pub trait RpcAPI {
    /// Salute given person's name
    async fn hello(name: String) -> String;

    /// Add two i32 numbers
    async fn sum_numbers(a: i32, b: i32) -> i32;

    /// Serialize/Deserialize CustomDataTypes
    async fn echo(foo: CustomDataType) -> CustomDataType;
}
