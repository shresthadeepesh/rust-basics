use tokio::net::TcpStream;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

use crate::models::post::Post;
use std::error::Error;
use std::net::SocketAddr;
use std::{fs, io};

pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("Server listeneing on port: 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];

    loop {
        let n = stream
            .read(&mut buf)
            .await
            .expect("Failed to read data from socket.");

        if n == 0 {
            return;
        }

        let request = String::from_utf8_lossy(&buf[0..n]);

        if let Some(first_line) = request.lines().next() {
            println!("{}", first_line);
        }

        let response = match &request {
            r if r.starts_with("GET /ping HTTP/1.1") => {
                let message = "Pong";
                let length = message.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{message}",
                    "text/html"
                )
            }
            r if r.starts_with("GET / HTTP/1.1") => {
                let contents = fs::read_to_string("views/index.html").unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{contents}",
                    "text/html"
                )
            }
            r if r.starts_with("GET /api/posts HTTP/1.1") => {
                let posts = Post::get_posts().await.unwrap();

                let contents = serde_json::to_string(&posts).unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{contents}",
                    "application/json"
                )
            }
            r if r.starts_with("GET /api/posts/1 HTTP/1.1") => {
                let posts = Post::get_post(1, false).await.unwrap();

                let contents = serde_json::to_string(&posts).unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{contents}",
                    "application/json"
                )
            }
            r if r.starts_with("POST /api/posts HTTP/1.1") => {
                let post = Post {
                    id: 101,
                    title: String::from("Github rust blog."),
                    body: String::from("Github rust blog."),
                    userId: 1,
                    user: None,
                };

                let contents = serde_json::to_string(&post).unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{contents}",
                    "application/json"
                )
            }
            _ => {
                let contents = fs::read_to_string("views/404.html").unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\nContent-Type:{}\r\n\n{contents}",
                    "text/html")
            }
        };

        stream
            .write_all(response.as_bytes())
            .await
            .expect("Failed to write data to socket.");
    }
}

async fn cli_blog_app() {
    println!("Welcome to blog app...");

    let mut post_id = String::new();
    let mut cont = String::new();
    loop {
        println!("Please enter a postId: ");

        io::stdin()
            .read_line(&mut post_id)
            .expect("Failed to read line.");

        let post_id: u32 = match post_id.trim().parse() {
            Ok(post_id) => post_id,
            Err(_) => continue,
        };

        println!("Fetching post of id: {post_id}");

        let post = Post::get_post(post_id, true).await;

        println!("{:#?}", post);

        println!("Do you want to continue? (y/n)");

        io::stdin()
            .read_line(&mut cont)
            .expect("Failed to read line.");

        if cont.trim() != "y" {
            break;
        }
    }
}
