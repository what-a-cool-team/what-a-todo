use domain::models::greeting::Greeting;
use domain::models::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGreetingsResponse {
    pub greetings: Vec<Greeting>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGreetingResponse {
    pub greeting: Greeting,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUserResponse {
    pub user: User,
}