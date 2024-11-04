use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateGreetingRequest {
    #[validate(length(min = 1))]
    pub greeting: String,
}

#[derive(serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
