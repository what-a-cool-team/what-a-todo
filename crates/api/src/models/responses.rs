use domain::models::greeting::Greeting;
use domain::models::task::Task;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGreetingsResponse {
    pub greetings: Vec<Greeting>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGreetingResponse {
    pub greeting: Greeting,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetTasksResponse {
    pub tasks: Vec<Task>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTaskResponse {
    pub task: Task,
}
