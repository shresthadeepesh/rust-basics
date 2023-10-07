use crate::models::post::Post;
use log::info;
use rusqlite::{params, Connection, Params};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

pub async fn poll(
    db_connection: Arc<Mutex<Connection>>,
) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let mut connection = db_connection.lock().await;
    let posts = Post::get_posts().await?;

    // let mut prepare = connection.prepare("SELECT * from posts")?;
    // let test = prepare
    //     .query_map((), |row| {
    //         Ok(Post {
    //             id: row.get(0)?,
    //             title: row.get(1)?,
    //             body: row.get(2)?,
    //             userId: row.get(3)?,
    //             user: None,
    //         })
    //     })
    //     .unwrap();

    // for p in test {
    //     println!("{:?}", p.unwrap());
    // }

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

pub async fn get_posts(
    db_connection: Arc<Mutex<Connection>>,
) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
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
) -> Result<Post, Box<dyn std::error::Error>> {
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
) -> Result<Post, Box<dyn std::error::Error>> {
    let connection = db_connection.lock().await;

    info!("Inserting post into the database....");
    let query = "INSERT INTO posts(title, body, userId) VALUES(?1, ?2, ?3);";

    let mut stmt = connection.prepare(query).unwrap();
    stmt.execute(params![post.title, post.body, post.userId])
        .unwrap();

    info!("Insertion completed.");

    Ok(post)
}
