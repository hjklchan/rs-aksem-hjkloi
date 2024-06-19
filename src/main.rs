use axum::{http::Method, Router};
use rs_aksem_hjkoi::{app_state, config, handler, must_connect_pool};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化 env 配置
    config::init();

    // 初始化 log
    tracing_subscriber::fmt::init();

    // 创建数据库连接池
    // TODO: database_url 应该通过配置或 **全局静态** 配置获取
    let database_url = config::get("DATABASE_URL");
    let pool = must_connect_pool(database_url).await;

    // 初始化 Axum 全局状态
    let app_state = app_state::new(pool);
    // 实例化路由
    let routes =
        handler::routes(app_state).layer(CorsLayer::new().allow_origin(cors::Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]));
    // 实例化 App
    let app = Router::new().merge(routes);

    // Tcp 监听器
    // TODO: addr 应该通过配置或 **全局静态** 配置获取
    let port = config::get("SERVER_PORT");
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let tcp_listener = TcpListener::bind(addr).await?;
    // TODO: 应该使用日志库打印
    info!("Listen on http://{}", addr.to_string());

    // 启动服务
    axum::serve(tcp_listener, app).await?;

    Ok(())
}
