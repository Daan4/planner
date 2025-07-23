use dioxus::prelude::*;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use super::model::Task;
use chrono::Utc;
use super::schema::tasks;
use std::env;
use dotenvy::dotenv;

async fn get_db_connection() -> Result<SyncConnectionWrapper<SqliteConnection>, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SyncConnectionWrapper::<SqliteConnection>::establish(&database_url).await
}

async fn create_task(task: String) -> Result<Task, ServerFnError> {
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let new_task = Task {
        id: 0, // This will be auto-incremented by the database
        content: task,
        created_at: Utc::now().naive_utc(),
        modified_at: None,
        deleted_at: None,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database insert error: {}", e)))?;
    
    Ok(new_task)
}