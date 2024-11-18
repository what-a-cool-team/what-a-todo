use std::sync::Arc;

use crate::models::task::Task;
use axum::async_trait;
use chrono::Utc;
use sqlx::postgres::PgRow;
use sqlx::{query, Pool, Postgres, Row};

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
            WITH tags_array AS (
                SELECT
                    tt.task_id,
                    COALESCE(ARRAY_AGG(tg.name), ARRAY[]::VARCHAR[]) AS tags
                FROM tasks_tags AS tt
                JOIN tags AS tg ON tt.tag_id=tg.id
                GROUP BY tt.task_id
            )

            SELECT
                t.id,
                t.user_id,
                t.name,
                t.description,
                t.status,
                t.created_at,
                t.updated_at,
                ta.tags
            FROM tasks AS t
            LEFT JOIN tags_array AS ta ON t.id = ta.task_id
            ",
        )
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
            INSERT INTO tasks (user_id, name, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
        )
            .bind(task.user_id)
            .bind(task.name)
            .bind(task.description)
            .bind(task.status)
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
                    ",
                )
                    .bind(tag_name)
                    .fetch_one(&mut *transaction)
                    .await?;

                let tag_id: i32 = tag_row.try_get("id")?;

                query(
                    "
                    INSERT INTO tasks_tags (task_id, tag_id)
                    VALUES ($1, $2)
                    ",
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
