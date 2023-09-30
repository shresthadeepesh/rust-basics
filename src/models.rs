pub mod post;
pub mod user;

pub enum Endpoints {
    GetPosts,
    GetPost(u32),
    GetTodos,
    GetUsers,
    GetUser(u32),
}

impl Endpoints {
    pub fn base_url(endpoints: Endpoints) -> String {
        let base_url = String::from("https://jsonplaceholder.typicode.com");

        match endpoints {
            Endpoints::GetPosts => base_url + "/posts/",
            Endpoints::GetPost(id) => format!("{base_url}/posts/{id}"),
            Endpoints::GetUsers => base_url + "/users/",
            Endpoints::GetUser(id) => format!("{base_url}/users/{id}"),
            Endpoints::GetTodos => base_url + "/todos/",
        }
    }
}
