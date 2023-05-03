use super::{label::Label, RepositoryError};
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use validator::Validate;

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity>;
    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity>;
    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;

        let row = sqlx::query_as::<_, TodoFromRow>(
            r#"
                insert into todos (text, completed)
                values ($1, false)
                returning *
            "#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await?;

        sqlx::query(
            r#"
                insert into todo_labels (todo_id, label_id)
                select $1, id
                from unnest($2) as t(id)
            "#,
        )
        .bind(row.id)
        .bind(payload.labels)
        .execute(&self.pool)
        .await?;

        tx.commit().await?;
        Ok(self.find(row.id).await?)
    }

    async fn find(&self, id: i32) -> anyhow::Result<TodoEntity> {
        let todos = fold_entities(
            sqlx::query_as::<_, TodoWithLabelFromRow>(
                r#"
                    select todos.*, labels.id as label_id, labels.name as label_name from todos
                    left outer join todo_labels on todos.id = todo_labels.todo_id
                    left outer join labels on labels.id = todo_labels.label_id
                    where todos.id = $1
                "#,
            )
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?,
        );
        Ok(todos.first().ok_or(RepositoryError::NotFound(id))?.clone())
    }

    async fn all(&self) -> anyhow::Result<Vec<TodoEntity>> {
        Ok(fold_entities(
            sqlx::query_as::<_, TodoWithLabelFromRow>(
                r#"
                    select todos.*, labels.id as label_id, labels.name as label_name from todos
                    left outer join todo_labels t1 on todos.id = t1.todo_id
                    left outer join labels on labels.id = t1.label_id
                    order by todos.id desc
                "#,
            )
            .fetch_all(&self.pool)
            .await?,
        ))
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<TodoEntity> {
        let tx = self.pool.begin().await?;
        let old_todo = self.find(id).await?;

        sqlx::query(
            r#"
                update todos set text = $1, completed = $2
                where id = $3
                returning *
            "#,
        )
        .bind(payload.text.unwrap_or(old_todo.text))
        .bind(payload.completed.unwrap_or(old_todo.completed))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if let Some(labels) = payload.labels {
            sqlx::query(
                r#"
                    delete from todo_labels where todo_id = $1
                "#,
            )
            .bind(id)
            .execute(&self.pool)
            .await?;

            sqlx::query(
                r#"
                    insert into todo_labels (todo_id, label_id)
                    select $1, id
                    from unnest($2) as t(id)
                "#,
            )
            .bind(id)
            .bind(labels)
            .execute(&self.pool)
            .await?;
        };

        tx.commit().await?;
        Ok(self.find(id).await?)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let tx = self.pool.begin().await?;
        sqlx::query(
            r#"
                delete from todo_labels
                where todo_id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        sqlx::query(
            r#"
                delete from todos
                where id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        tx.commit().await?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, FromRow)]
pub struct TodoFromRow {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, FromRow)]
pub struct TodoWithLabelFromRow {
    id: i32,
    text: String,
    completed: bool,
    label_id: Option<i32>,
    label_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

fn fold_entities(rows: Vec<TodoWithLabelFromRow>) -> Vec<TodoEntity> {
    let mut accum: Vec<TodoEntity> = vec![];

    'outer: for row in rows.iter() {
        for todo in accum.iter_mut() {
            if todo.id == row.id {
                todo.labels.push(Label {
                    id: row.label_id.unwrap(),
                    name: row.label_name.clone().unwrap(),
                });
                continue 'outer;
            }
        }

        let labels = if row.label_id.is_some() {
            vec![Label {
                id: row.label_id.unwrap(),
                name: row.label_name.clone().unwrap(),
            }]
        } else {
            vec![]
        };

        accum.push(TodoEntity {
            id: row.id,
            text: row.text.clone(),
            completed: row.completed,
            labels,
        });
    }

    accum
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: String,
    labels: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: Option<String>,
    completed: Option<bool>,
    labels: Option<Vec<i32>>,
}
