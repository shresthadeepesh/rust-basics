use crate::models::user::User;
use serde::{Deserialize, Serialize};

use super::Endpoints;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub userId: u32,
    pub user: Option<User>,
}

impl Post {
    pub fn new(id: u32, title: String, body: String, userId: u32, user: Option<User>) -> Self {
        Post {
            id,
            title,
            body,
            userId,
            user,
        }
    }

    pub async fn get_posts() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let posts = reqwest::get(Endpoints::base_url(Endpoints::GetPosts))
            .await?
            .json::<Vec<Post>>()
            .await?;

        Ok(posts)
    }

    pub async fn get_post(id: u32, load_user: bool) -> Result<Post, Box<dyn std::error::Error>> {
        let url = Endpoints::base_url(Endpoints::GetPost(id));
        let mut post = reqwest::get(url).await?.json::<Post>().await?;

        if load_user {
            let user = User::get_user(post.userId).await?;
            post = Post {
                user: Some(user),
                ..post
            }
        }

        Ok(post)
    }
}
