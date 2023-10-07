extern crate dotenv;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::env;

use crate::models::post::Post;
use crate::services::post_service::{self, poll};
use log::info;
use rusqlite::Connection;
use std::error::Error;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use hyper::{Body, Request, Response, Server};

pub mod models;
pub mod services;

async fn handle_hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let contents = fs::read_to_string("views/index.html").unwrap();
    let response = Response::new(Body::from(contents));
    Ok(response)
}

async fn handle_ping(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = "Pong";
    let response = Response::new(Body::from(response_body));
    Ok(response)
}

async fn handle_not_found(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = "404 Not Found";
    let response = Response::builder()
        .status(404)
        .body(Body::from(response_body))
        .unwrap();
    Ok(response)
}

async fn get_db_posts(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let posts = post_service::get_posts(connection.clone()).await;

    let contents = serde_json::to_string(&posts.unwrap()).unwrap();
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(contents))
        .unwrap();
    Ok(response)
}

async fn get_posts(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let posts = Post::get_posts().await;

    let contents = serde_json::to_string(&posts.unwrap()).unwrap();
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(contents))
        .unwrap();
    Ok(response)
}

async fn poll_posts(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let t = poll(connection.clone()).await;

    let message = r#"
                    {
                        message: "Polling completed."
                    }
                "#;

    let contents = serde_json::to_string(message).unwrap();
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(contents))
        .unwrap();
    Ok(response)
}

async fn router(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => handle_hello(req).await,
        (&hyper::Method::GET, "/ping") => handle_ping(req).await,
        (&hyper::Method::GET, "/api/posts/db") => get_db_posts(req, connection).await,
        (&hyper::Method::GET, "/api/posts") => get_posts(req).await,
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;

    Ok(())
}
