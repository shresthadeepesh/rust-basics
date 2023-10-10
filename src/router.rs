use crate::models::post::Post;
use crate::services::post_service::{self, poll};
use hyper::{Body, Request, Response};
use log::{debug, info};
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

pub async fn get_post(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let post_id = get_post_id(req);

    if post_id != 0 {
        let post = Post::get_post(post_id, false).await.unwrap();

        let contents = serde_json::to_string(&post).unwrap();
        let response = Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from(contents))
            .unwrap();
        return Ok(response);
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

    let _ = post_service::create_post(db_connection, post).await;

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

pub async fn delete_post(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let post_id = get_post_id(req);

    if post_id != 0 {
        let post = post_service::delete_post(connection, post_id)
            .await
            .unwrap();

        if post {
            let message = r#"{"message": "Post has been deleted successfully."}"#;
            let response = Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(message))
                .unwrap();

            return Ok(response);
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

pub async fn update_post(
    req: Request<Body>,
    connection: Arc<Mutex<Connection>>,
) -> Result<Response<Body>, Infallible> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let post: Post = serde_json::from_str(&body_str).unwrap();

    let _ = post_service::update_post(connection, post).await.unwrap();

    let message = r#"{"message": "Post has been updated successfully."}"#;
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(message))
        .unwrap();

    return Ok(response);
}

fn get_post_id(req: Request<Body>) -> u32 {
    let path = req.uri().path();
    let path_segments: Vec<&str> = path.split('/').collect();

    if path_segments.len() >= 3 {
        if let Ok(post_id) = u32::from_str_radix(path_segments[3], 10) {
            debug!("{}", post_id);
            return post_id;
        }
    }

    info!("Cannot parse post_id from the url.");

    0
}
