use super::Endpoints;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Geo {
    lat: String,
    lng: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

impl User {
    pub async fn get_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let users = reqwest::get(Endpoints::base_url(Endpoints::GetUsers))
            .await?
            .json::<Vec<User>>()
            .await?;

        Ok(users)
    }

    pub async fn get_user(id: u32) -> Result<User, Box<dyn std::error::Error>> {
        let url = Endpoints::base_url(Endpoints::GetUser(id));
        let user = reqwest::get(url).await?.json::<User>().await?;

        Ok(user)
    }
}
