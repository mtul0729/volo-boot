use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};

use user::S;

#[volo::main]
async fn main() {
    // 这里不要使用 `let _ = xxx;` 的形式来接受返回结果，避免被立即drop掉导致日志声明周期有问题
    let _logger_guard = common::logger::init_tracing();

    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    Server::new()
        .add_service(ServiceBuilder::new(volo_gen::user::UserServiceServer::new(S)).build())
        .run(addr)
        .await
        .unwrap();
}
