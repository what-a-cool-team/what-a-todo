use crate::models::requests::CreateTaskRequest;
use crate::models::responses::{CreateTaskResponse, GetTasksResponse};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_valid::Valid;
use domain::errors::ApiResult;
use domain::models::status::Status;
use domain::models::task::Task;
use domain::services::service_registry::ServiceRegistry;
use domain::services::task_service::DynTaskService;

pub struct TasksController;

impl TasksController {
    pub fn new_router(service_registry: ServiceRegistry) -> Router {
        Router::new()
            .route("/", get(Self::get_tasks))
            .route("/", post(Self::create_task))
            .layer(Extension(service_registry.task_service))
    }

    pub async fn get_tasks(
        Extension(task_service): Extension<DynTaskService>,
    ) -> ApiResult<Json<GetTasksResponse>> {
        Ok(Json(GetTasksResponse {
            tasks: task_service.get_tasks().await?,
        }))
    }

    pub async fn create_task(
        Extension(task_service): Extension<DynTaskService>,
        Valid(Json(request)): Valid<Json<CreateTaskRequest>>,
    ) -> ApiResult<Json<CreateTaskResponse>> {
        Ok(Json(CreateTaskResponse {
            task: task_service
                .create_task(Task::new(
                    1, // user_id will be set once auth is done.
                    request.name,
                    request.description,
                    Status::Created,
                    request.tags,
                ))
                .await?,
        }))
    }
}
