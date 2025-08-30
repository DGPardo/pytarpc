#[tarpc::service]
pub trait RpcAPI {
    async fn hello(_name: String) -> String;
}
