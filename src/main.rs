use crate::models::post::Post;

pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // let posts = Post::get_posts().await?;

    // println!("{:#?}", posts);

    let post = Post::get_post(1, true).await?;

    println!("{:#?}", post);

    Ok(())
}
