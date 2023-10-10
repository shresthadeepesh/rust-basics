use crate::models::post::Post;
use log::info;
use rusqlite::{params, Connection};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

pub async fn poll(db_connection: Arc<Mutex<Connection>>) -> Result<Vec<Post>, Box<dyn Error>> {
    let mut connection = db_connection.lock().await;
    let posts = Post::get_posts().await?;

    if posts.len() > 0 {
        let transaction = connection.transaction().unwrap();

        info!("Truncating Table posts...");
        let query = "DELETE FROM posts;";
        transaction.execute(query, ()).unwrap();

        info!("Truncate successfull.");

        let mut statement: Vec<String> = Vec::new();

        statement.push(String::from(
            "INSERT INTO posts(title, body, userId) VALUES",
        ));

        for (index, post) in posts.iter().enumerate() {
            if index == posts.len() - 1 {
                statement.push(format!(
                    "('{}','{}','{}');",
                    post.title, post.body, post.userId
                ));
            } else {
                statement.push(format!(
                    "('{}','{}','{}'),",
                    post.title, post.body, post.userId
                ));
            }
        }

        let statement = statement.join("");

        info!("Inserting fetched posts to the database.");
        transaction.execute(statement.as_str(), ()).unwrap();

        transaction.commit().unwrap();
        info!("Insertion completed.");
    }

    Ok(posts)
}

pub async fn get_posts(db_connection: Arc<Mutex<Connection>>) -> Result<Vec<Post>, Box<dyn Error>> {
    let connection = db_connection.lock().await;

    info!("Pulling posts from the database...");
    let mut prepare = connection.prepare("SELECT * from posts")?;
    let posts: Result<Vec<Post>, _> = prepare
        .query_map((), |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                body: row.get(2)?,
                userId: row.get(3)?,
                user: None,
            })
        })
        .unwrap()
        .collect();

    info!("Posts pulled successfully.");

    Ok(posts.unwrap())
}

pub async fn get_post(
    db_connection: Arc<Mutex<Connection>>,
    post_id: u32,
) -> Result<Post, Box<dyn Error>> {
    let connection = db_connection.lock().await;

    info!("Pulling posts from the database...");
    let mut prepare = connection.prepare("SELECT * from posts WHERE id = ?1")?;
    let post: Result<Post, rusqlite::Error> = prepare.query_row(params![post_id], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            body: row.get(2)?,
            userId: row.get(3)?,
            user: None,
        })
    });

    info!("Posts pulled successfully.");

    Ok(post.unwrap())
}

pub async fn create_post(
    db_connection: Arc<Mutex<Connection>>,
    post: Post,
) -> Result<Post, Box<dyn Error>> {
    let connection = db_connection.lock().await;

    info!("Inserting post into the database....");
    let query = "INSERT INTO posts(title, body, userId) VALUES(?1, ?2, ?3);";

    let mut stmt = connection.prepare(query).unwrap();
    stmt.execute(params![post.title, post.body, post.userId])
        .unwrap();

    info!("Insertion completed.");

    Ok(post)
}

pub async fn update_post(
    db_connection: Arc<Mutex<Connection>>,
    post: Post,
) -> Result<Post, Box<dyn Error>> {
    let connection = db_connection.lock().await;

    let db_post = get_post(db_connection.clone(), post.id).await;

    match db_post {
        Ok(db_post) => {
            info!("Updating post into the database....");
            let query = "UPDATE posts SET title = ?1, body = ?2, userId = ?3 WHERE id = ?4;";

            let mut stmt = connection.prepare(query).unwrap();
            stmt.execute(params![post.title, post.body, post.userId, post.id])
                .unwrap();

            info!("Update completed.");

            Ok(post)
        }
        Err(e) => {
            info!("Failed to update post, post not found.");
            Err(e)
        }
    }
}

pub async fn delete_post(
    db_connection: Arc<Mutex<Connection>>,
    post_id: u32,
) -> Result<bool, Box<dyn Error>> {
    let connection = db_connection.lock().await;

    let db_post = get_post(db_connection.clone(), post_id).await;

    match db_post {
        Ok(db_post) => {
            info!("Deleting post of id: {} from the database.", post_id);
            let query = "DELETE FROM posts WHERE id = ?1";
            let mut stmt = connection.prepare(query).unwrap();
            let result = stmt.execute(params![post_id]).unwrap();

            info!("Post has been deleted.");

            if result > 0 {
                return Ok(true);
            }

            Ok(false)
        }
        Err(e) => {
            info!("Failed to update post, post not found.");
            Err(e)
        }
    }
}
