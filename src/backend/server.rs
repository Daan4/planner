use dioxus::prelude::*;
#[cfg(feature = "server")]
use diesel::prelude::*;
#[cfg(feature = "server")]
use diesel_async::{RunQueryDsl, AsyncConnection};
#[cfg(feature = "server")]
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use super::model::*;
#[cfg(feature = "server")]
use chrono::Utc;
use chrono::NaiveDate;
use std::env;
#[cfg(feature = "server")]
use dotenvy::dotenv;
#[cfg(feature = "server")]
use uuid::Uuid;
#[cfg(feature = "server")]
use std::sync::LazyLock;
#[cfg(feature = "server")]
use tokio::sync::Mutex;

#[cfg(feature = "server")]
static DB_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[cfg(feature = "server")]
async fn get_db_connection() -> Result<SyncConnectionWrapper<SqliteConnection>, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SyncConnectionWrapper::<SqliteConnection>::establish(&database_url).await
}

#[server]
pub async fn create_task(title: String, date: Option<NaiveDate>) -> Result<Task, ServerFnError> {
    use super::schema::tasks;

    let mut new_task = Task {
        id: Id(Uuid::now_v7()),
        title: title,
        important: false,
        urgent: false,
        content: None,
        completed: false,
        role_id: None,
        backlog_id: None,
        scheduled_date: None,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
        deleted_at: None,
    };

    if let Some(date) = date {
        new_task.scheduled_date = Some(date);
    }

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database insert error: {}", e)))?;

    Ok(new_task)
}

#[server]
pub async fn get_tasks(date: Option<NaiveDate>) -> Result<Vec<Task>, ServerFnError> {
    use super::schema::tasks::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    match date {
        Some(date) => {
            let taskvec = tasks
                .select(Task::as_select())
                .filter(deleted_at.is_null().and(scheduled_date.eq(date)))
                .load(&mut conn)
                .await
                .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

            Ok(taskvec)
        },
        None => {
            let taskvec = tasks
                .select(Task::as_select())
                .filter(deleted_at.is_null().and(scheduled_date.is_null()))
                .load(&mut conn)
                .await
                .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

            Ok(taskvec)
        }
    }

}

#[server]
pub async fn update_task(task: Task) -> Result<Task, ServerFnError> {
    use super::schema::tasks::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let task = diesel::update(tasks.find(task.id))
        .set((
            title.eq(task.title), 
            important.eq(task.important),
            urgent.eq(task.urgent),
            content.eq(task.content),
            completed.eq(task.completed),
            role_id.eq(task.role_id),
            backlog_id.eq(task.backlog_id),
            scheduled_date.eq(task.scheduled_date),
            updated_at.eq(Utc::now().naive_utc())))
        .returning(Task::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(task)
}

#[server]
pub async fn delete_task(task_id: Id) -> Result<(), ServerFnError> {
    use super::schema::tasks::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::update(tasks.find(task_id))
        .set(deleted_at.eq(Utc::now().naive_utc()))
        .returning(Task::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(())
}

#[server]
pub async fn create_backlog(name: String) -> Result<Backlog, ServerFnError> {
    use super::schema::backlogs;

    let new_backlog = Backlog {
        id: Id(Uuid::now_v7()),
        name
    };

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::insert_into(backlogs::table)
        .values(&new_backlog)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database insert error: {}", e)))?;
    Ok(new_backlog)
}

#[server]
pub async fn get_backlogs() -> Result<Vec<Backlog>, ServerFnError> {
    use super::schema::backlogs::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let backlogvec = backlogs
        .select(Backlog::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(backlogvec)
}

#[server]
pub async fn update_backlog(backlog: Backlog) -> Result<(), ServerFnError> {
    use super::schema::backlogs::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::update(backlogs.find(backlog.id))
        .set(name.eq(backlog.name))
        .returning(Backlog::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(())
}

#[server]
pub async fn delete_backlog(backlog_id: Id) -> Result<(), ServerFnError> {
    use super::schema::backlogs::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::delete(backlogs
        .filter(id.eq(backlog_id.0.to_string())))
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database delete error: {}", e)))?;

    Ok(())
}


#[server]
pub async fn create_role(name: String) -> Result<Role, ServerFnError> {
    use super::schema::roles;

    let new_role = Role {
        id: Id(Uuid::now_v7()),
        name
    };

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::insert_into(roles::table)
        .values(&new_role)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database insert error: {}", e)))?;
    Ok(new_role)
}

#[server]
pub async fn get_roles() -> Result<Vec<Role>, ServerFnError> {
    use super::schema::roles::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    let rolesvec = roles
        .select(Role::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(rolesvec)
}

#[server]
pub async fn update_role(role: Role) -> Result<(), ServerFnError> {
    use super::schema::roles::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::update(roles.find(role.id))
        .set(name.eq(role.name))
        .returning(Role::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database fetch error: {}", e)))?;

    Ok(())
}

#[server]
pub async fn delete_role(role_id: Id) -> Result<(), ServerFnError> {
    use super::schema::roles::dsl::*;

    let _guard = DB_MUTEX.lock().await;
    let mut conn = get_db_connection().await.map_err(|e| ServerFnError::new(format!("Database connection error: {}", e)))?;

    diesel::delete(roles
        .filter(id.eq(role_id.0.to_string())))
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("Database delete error: {}", e)))?;

    Ok(())
}
