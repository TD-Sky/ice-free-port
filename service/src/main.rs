mod handlers;
mod models;
mod reply;
mod router;
mod state;
mod errors;
mod middleware;

use self::router::router;
use self::state::State;
use dotenv::dotenv;
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Server};
use std::env;
use std::io::Result as IoResult;

#[tokio::main]
async fn main() -> IoResult<()> {
    env::set_var("RUST_LOG", "poem=debug");
    let _ = dotenv();
    let db_url = env::var("DATABASE_URL").expect("lose DATABASE_URL");
    let server_url = env::var("SERVER_URL").expect("lose SERVER_URL");

    tracing_subscriber::fmt::init();

    // 数据库连接池
    let db = sea_orm::Database::connect(&db_url).await.unwrap();

    // 服务器状态
    let state = State::new(db);

    // 服务器
    let app = router().with(Tracing).data(state);

    let server = Server::new(TcpListener::bind(server_url)).name("ice-free-port");
    server.run(app).await
}
