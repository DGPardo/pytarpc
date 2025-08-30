use pytarpc::tarpc_python_client;

#[tarpc_python_client]
#[tarpc::service]
pub trait RpcAPI {
    async fn hello(_name: String) -> String;

    async fn sum_numbers(a: i32, b: i32) -> i32;
}
