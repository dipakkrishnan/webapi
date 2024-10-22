use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub name: String,
}
