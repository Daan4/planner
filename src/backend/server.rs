use dioxus::prelude::*;
#[cfg(feature = "server")]
use diesel::prelude::*;
#[cfg(feature = "server")]
use diesel_async::{RunQueryDsl, AsyncConnection};
#[cfg(feature = "server")]
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use super::model::{Task, Id};
#[cfg(feature = "server")]
use chrono::Utc;
use std::env;
#[cfg(feature = "server")]
use dotenvy::dotenv;
#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "server")]
async fn get_db_connection() -> Result<SyncConnectionWrapper<SqliteConnection>, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SyncConnectionWrapper::<SqliteConnection>::establish(&database_url).await
}

#[server]
pub async fn create_task(task_content: String) -> Result<Task, ServerFnError> {
    use super::schema::tasks;

    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let new_task = Task {
        id: Id(Uuid::now_v7()), // This will be auto-incremented by the database
        content: task_content,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
        deleted_at: None,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database insert error: {}", e)))?;
    
    Ok(new_task)
}

#[server]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    use super::schema::tasks::dsl::*;

    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let taskvec = tasks
        .select(Task::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

        Ok(taskvec)
}

#[server]
pub async fn update_task(task_id: Id, task_content: String) -> Result<Task, ServerFnError> {
    use super::schema::tasks::dsl::*;

    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let task = diesel::update(tasks.find(task_id))
        .set((content.eq(task_content), updated_at.eq(Utc::now().naive_utc())))
        .returning(Task::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(task)
}

#[server]
pub async fn delete_task(task_id: Id) -> Result<(), ServerFnError> {
    use super::schema::tasks::dsl::*;

    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::delete(tasks
        .filter(id.eq(task_id.0.to_string())))
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database delete error: {}", e)))?;

    Ok(())
}