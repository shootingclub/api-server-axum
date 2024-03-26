use crate::entity::{CreateUser, User};

pub fn get_users() -> Vec<User> {
    let mut users: Vec<User> = Vec::new();
    users.push(User {
        id: 1222,
        username: String::from("kak"),
    });
    return users;
}

pub fn create_users(user: CreateUser) -> User {
    User {
        id: 1377,
        username: user.username,
    }
}