use std::sync::Arc;
use axum::async_trait;
use chrono::Utc;
use sqlx::{Pool, Postgres, query, Row};
use sqlx::postgres::PgRow;
use crate::models::task::Task;

pub type DynTaskRepository = Arc<dyn TaskRepository + Send + Sync>;

#[async_trait]
pub trait TaskRepository {
    async fn get_tasks(&self) -> anyhow::Result<Vec<Task>>;
    async fn create_task(&self, task: Task) -> anyhow::Result<Task>;
}

#[derive(Clone)]
pub struct DomainTaskRepository {
    pool: Pool<Postgres>,
}

impl DomainTaskRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for DomainTaskRepository {
    async fn get_tasks(&self) -> anyhow::Result<Vec<Task>> {
        let tasks: Vec<Task> = query(
            "
                SELECT
                    tasks.id,
                    tasks.user_id,
                    tasks.name,
                    tasks.description,
                    tasks.status,
                    tasks.created_at,
                    tasks.updated_at,
                    COALESCE(array_agg(tags.name), ARRAY[]::VARCHAR[]) AS tags
                FROM tasks
                LEFT JOIN tasks_tags ON tasks.id = tasks_tags.task_id
                LEFT JOIN tags ON tasks_tags.tag_id = tags.id
                GROUP BY tasks.id
                ORDER BY tasks.id
            ")
                .map(|row: PgRow| Task {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    status: row.get("status"),
                    tags: row.get("tags"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
                .fetch_all(&self.pool)
                .await?;
        Ok(tasks)
    }

    async fn create_task(&self, task: Task) -> anyhow::Result<Task> {
        let now = Utc::now();

        let name = task.name.clone();
        let description = task.description.clone();
        let status = task.status.clone();

        // Start a transaction to ensure atomicity
        let mut transaction = self.pool.begin().await?;

        let task_row = query(
            "
            INSERT INTO tasks (id, user_id, name, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            ",
        )
            .bind(task.id)
            .bind(task.user_id)
            .bind(task.name)
            .bind(task.description)
            .bind(task.status)
            .bind(now)
            .bind(now)
            .fetch_one(&mut *transaction)
            .await?;

        let task_id = task_row.try_get::<i32, _>("id")?;

        if let Some(tag_names) = &task.tags {
            for tag_name in tag_names {
                let tag_row = query(
                    "
                    INSERT INTO tags (name)
                    VALUES ($1)
                    RETURNING id
                    "
                )
                    .bind(tag_name)
                    .fetch_one(&mut *transaction)
                    .await?;

                let tag_id: i32 = tag_row.try_get("id")?;

                query(
                    "
                    INSERT INTO tasks_tags (task_id, tag_id)
                    VALUES ($1, $2)
                    "
                )
                    .bind(task_id)
                    .bind(tag_id)
                    .execute(&mut *transaction)
                    .await?;
            }
        }

        // Commit the transaction
        transaction.commit().await?;

        let created_task = Task::new(task.user_id, name, description, status, task.tags);
        Ok(created_task)
    }
}
