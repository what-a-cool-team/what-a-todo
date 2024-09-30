use std::sync::Arc;

use axum::async_trait;

use crate::errors::ApiResult;
use crate::models::task::Task;
use crate::repositories::task_repository::DynTaskRepository;

pub type DynTaskService = Arc<dyn TaskService + Send + Sync>;

#[async_trait]
pub trait TaskService {
    async fn get_tasks(&self) -> ApiResult<Vec<Task>>;
    async fn create_task(&self, task: Task) -> ApiResult<Task>;
}

#[derive(Clone)]
pub struct DomainTaskService {
    pub task_repository: DynTaskRepository,
}

impl DomainTaskService {
    pub fn new(task_repository: DynTaskRepository) -> Self {
        Self {
            task_repository,
        }
    }
}

#[async_trait]
impl TaskService for DomainTaskService {
    async fn get_tasks(&self) -> ApiResult<Vec<Task>> {
        Ok(self.task_repository.get_tasks().await?)
    }

    async fn create_task(&self, task: Task) -> ApiResult<Task> {
        Ok(self
            .task_repository
            .create_task(Task::from(task))
            .await?)
    }
}
