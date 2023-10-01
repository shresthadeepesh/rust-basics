use crate::models::post::Post;
use std::io;

pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
