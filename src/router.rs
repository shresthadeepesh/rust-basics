use crate::models::post::Post;
use crate::services::post_service::{self, poll};
use hyper::{Body, Request, Response};
use rusqlite::Connection;
use std::convert::Infallible;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let contents = fs::read_to_string("views/index.html").unwrap();
    let response = Response::new(Body::from(contents));
    Ok(response)
}

pub async fn handle_ping(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = "Pong";
    let response = Response::new(Body::from(response_body));
    Ok(response)
}

pub async fn handle_not_found(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response: Response<Body>;
    if req.headers().contains_key("accept")
        && req.headers().get("accept").unwrap() == "application/json"
    {
        let message = r#"{"message": "Not found."}"#;
        response = Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .body(Body::from(message))
            .unwrap();
    } else {
        let contents = fs::read_to_string("views/404.html").unwrap();
        response = Response::builder()
            .status(404)
            .body(Body::from(contents))
            .unwrap();
    }

    Ok(response)
}

pub async fn get_db_posts(
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

pub async fn get_posts(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let posts = Post::get_posts().await;

    let contents = serde_json::to_string(&posts.unwrap()).unwrap();
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(contents))
        .unwrap();
    Ok(response)
}

pub async fn poll_posts(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let _ = poll(connection.clone()).await;

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
