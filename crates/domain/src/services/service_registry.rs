use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::repositories::greeting_repository::{DomainGreetingRepository, DynGreetingRepository};
use crate::repositories::task_repository::{DomainTaskRepository, DynTaskRepository};
use crate::services::greeting_service::{DomainGreetingService, DynGreetingService};
use crate::services::task_service::{DomainTaskService, DynTaskService};
use crate::repositories::user_repository::{DomainUserRepository, DynUserRepository};
use crate::services::user_service::{DomainUserService, DynUserService};
use settings::settings::Settings;

#[derive(Clone)]
pub struct ServiceRegistry {
    pub greeting_service: DynGreetingService,
    pub task_service: DynTaskService,
    pub user_service: DynUserService,
    pub auth_service: settings::settings::Auth,
}

impl ServiceRegistry {
    pub fn new(pool: Pool<Postgres>, settings: Settings) -> Self {
        let auth_service = settings.auth;
        let greeting_repository =
            Arc::new(DomainGreetingRepository::new(pool.clone())) as DynGreetingRepository;
        let greeting_service =
            Arc::new(DomainGreetingService::new(greeting_repository.clone())) as DynGreetingService;
        let task_repository =
            Arc::new(DomainTaskRepository::new(pool.clone())) as DynTaskRepository;
        let task_service =
            Arc::new(DomainTaskService::new(task_repository.clone())) as DynTaskService;
        let user_repository =
            Arc::new(DomainUserRepository::new(pool.clone(), auth_service.clone())) as DynUserRepository;
        let user_service = 
            Arc::new(DomainUserService::new(user_repository.clone())) as DynUserService;

        ServiceRegistry { greeting_service, task_service, user_service, auth_service}
    }
}
