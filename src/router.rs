use crate::models::post::Post;
use crate::services::post_service::{self, poll};
use hyper::{Body, Request, Response};
use log::{debug, info};
use rusqlite::{params, Connection};
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

pub async fn get_post(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    let path_segments: Vec<&str> = path.split('/').collect();

    if path_segments.len() >= 3 {
        if let Ok(post_id) = u32::from_str_radix(path_segments[3], 10) {
            debug!("{}", post_id);
            let post = Post::get_post(post_id, false).await;

            match post {
                Ok(post) => {
                    let contents = serde_json::to_string(&post).unwrap();
                    let response = Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(Body::from(contents))
                        .unwrap();
                    return Ok(response);
                }
                Error => {
                    let message = r#"{"message": "Not found."}"#;
                    let response = Response::builder()
                        .status(404)
                        .header("Content-Type", "application/json")
                        .body(Body::from(message))
                        .unwrap();
                    return Ok(response);
                }
            }
        }
    }

    let message = r#"{"message": "Not found."}"#;
    let response = Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(Body::from(message))
        .unwrap();

    Ok(response)
}

pub async fn create_post(
    req: Request<Body>,
    db_connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    info!("Creating post....");

    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let post: Post = serde_json::from_str(&body_str).unwrap();

    let _ = post_service::create_post(db_connection, post);

    let message = r#"{"message": "Post created successfully."}"#;

    let response = Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(Body::from(message))
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
