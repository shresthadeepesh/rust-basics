pub mod post;
pub mod user;

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

pub enum Endpoints {
    GetPosts,
    GetPost(u32),
    GetTodos,
    GetUsers,
    GetUser(u32),
}

impl Endpoints {
    pub fn base_url(endpoints: Endpoints) -> String {
        match endpoints {
            Endpoints::GetPosts => format!("{BASE_URL}/posts"),
            Endpoints::GetPost(id) => format!("{BASE_URL}/posts/{id}"),
            Endpoints::GetUsers => format!("{BASE_URL}/users/"),
            Endpoints::GetUser(id) => format!("{BASE_URL}/users/{id}"),
            Endpoints::GetTodos => format!("{BASE_URL}/todos/"),
        }
    }
}
