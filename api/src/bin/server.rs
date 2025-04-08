use api::{build_router, consts, svc_discover, ServiceContext};
use clap::Parser;
use common::load_config::LoadConfig;
use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};
use user::user::{UserServiceClient};
use volo_http::{
    context::ServerContext,
    http::StatusCode,
    server::{layer::TimeoutLayer, Router, Server},
    Address,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}



fn timeout_handler(_: &ServerContext) -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Timeout!\n")
}

#[volo::main]
async fn main() {
    // 解析命令行参数, 启动命令如: cargo run --package api --bin server -- --config=/volo-boot/api/config/app_config.toml
    let args = Args::parse();

    // 全局日志模块初始化
    let _logger_guard = common::logger::init_tracing();

    // 加载配置
    let config_file_path = args.config;
    let app_config = api::app_config::AppConfig::load_toml(config_file_path.as_str()).unwrap();

    // 注册服务
    let nacos_config = app_config.sd.nacos;

    // 获取nacos naming service
    let nacos_naming_data = Arc::new(
        common::svc::nacos::build_naming_server(
            nacos_config.server_addr,
            nacos_config.namespace.unwrap_or("".to_string()),
            nacos_config.service_name.clone(),
            nacos_config.username,
            nacos_config.password,
        )
        .await
        .unwrap(),
    );

    // 注册
    let _nacos_svc_inst = common::svc::nacos::register_service(
        Arc::clone(&nacos_naming_data),
        nacos_config.service_name,
        app_config.port as i32,
        Default::default(),
    )
    .await;

    // 服务发现
    let discover = svc_discover::NacosDiscover {
        nacos_naming_data: nacos_naming_data.clone(),
    };

    let mut service_context = ServiceContext{ rpc_cli_user: None};

    // 订阅rpc服务
    if !app_config.subscribe_service.is_empty() {
        tracing::info!(
            "subscribe services: {}",
            app_config.subscribe_service.join(", ")
        );
        for sub_svc in app_config.subscribe_service {
            let sub_ret =
                common::svc::nacos::subscribe_service(nacos_naming_data.clone(), sub_svc.clone())
                    .await;
            match sub_ret {
                Ok(_) => {
                    tracing::info!("subscribe service: {} success.", sub_svc.clone());
                }
                Err(e) => {
                    tracing::error!("subscribe service: {} field, error: {}", sub_svc.clone(), e);
                }
            }

            // 构建grpc客户端
            match sub_svc.as_str() {
                consts::RPC_USER_KEY => {
                    let user_client: UserServiceClient = user::user::UserServiceClientBuilder::new(sub_svc)
                        .discover(discover.clone())
                        // .load_balance(volo::loadbalance::random::WeightedRandomBalance::new())
                        .load_balance(volo::loadbalance::consistent_hash::ConsistentHashBalance::new(Default::default()))
                        .build();
                    service_context.rpc_cli_user = Some(user_client);
                },
                _ => {}
            }

        }
    }

    // 启动http服务
    let app = Router::new()
        .merge(build_router(service_context))
        .layer(TimeoutLayer::new(Duration::from_secs(3), timeout_handler));

    let addr: SocketAddr = format!("[::]:{}", app_config.port).parse().unwrap();
    let addr = Address::from(addr);

    tracing::info!("Listening on {addr}");

    Server::new(app).run(addr).await.unwrap();
}
