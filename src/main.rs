extern crate dotenv;

use crate::router::{
    create_post, delete_post, get_db_posts, get_post, get_posts, handle_hello, handle_not_found,
    handle_ping, poll_posts, update_post,
};
use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use rusqlite::Connection;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::ctrl_c;
use tokio::sync::Mutex;

pub mod models;
pub mod router;
pub mod services;

async fn router(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    info!("[{}] {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => handle_hello(req).await,
        (&hyper::Method::GET, "/ping") => handle_ping(req).await,
        (&hyper::Method::GET, "/api/posts/db") => get_db_posts(req, connection).await,
        (&hyper::Method::GET, "/api/posts") => get_posts(req).await,
        (&hyper::Method::POST, "/api/posts") => create_post(req, connection).await,
        (&hyper::Method::DELETE, "/api/posts") => delete_post(req, connection).await,
        (&hyper::Method::PUT, "/api/posts") => update_post(req, connection).await,
        (&hyper::Method::GET, path) if path.starts_with("/api/posts/") => {
            get_post(req, connection).await
        }
        (&hyper::Method::POST, "/api/posts/poll") => poll_posts(req, connection).await,
        _ => handle_not_found(req).await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let log_stat = env::var("BASE_URL");

    info!("{}", log_stat.unwrap());

    let connection = Connection::open("db.sqlite").expect("Failed to connect to the database.");

    let router = Arc::new(router);
    let db_connection = Arc::new(Mutex::new(connection));

    let make_svc = make_service_fn(|_conn| {
        let conn = db_connection.clone();
        let router = router.clone();
        let service = service_fn(move |req| router(req, conn.clone()));
        async { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!("Server running at port: {}", addr.port());

    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(async {
            ctrl_c().await.expect("Failed to handle ctrl + c.");
            info!("Shutting down the server, closing all connections.");
        });
    server.await?;

    Ok(())
}
