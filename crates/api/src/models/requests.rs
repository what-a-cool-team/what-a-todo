use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateGreetingRequest {
    #[validate(length(min = 1))]
    pub greeting: String,
}

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}