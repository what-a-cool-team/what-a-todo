use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::repositories::greeting_repository::{DomainGreetingRepository, DynGreetingRepository};
use crate::repositories::task_repository::{DomainTaskRepository, DynTaskRepository};
use crate::services::greeting_service::{DomainGreetingService, DynGreetingService};
use crate::services::task_service::{DomainTaskService, DynTaskService};

#[derive(Clone)]
pub struct ServiceRegistry {
    pub greeting_service: DynGreetingService,
    pub task_service: DynTaskService,
}

impl ServiceRegistry {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let greeting_repository =
            Arc::new(DomainGreetingRepository::new(pool.clone())) as DynGreetingRepository;
        let greeting_service =
            Arc::new(DomainGreetingService::new(greeting_repository.clone())) as DynGreetingService;

        let task_repository =
            Arc::new(DomainTaskRepository::new(pool.clone())) as DynTaskRepository;
        let task_service =
            Arc::new(DomainTaskService::new(task_repository.clone())) as DynTaskService;

        ServiceRegistry {
            greeting_service,
            task_service,
        }
    }
}
