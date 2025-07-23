use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::schema::tasks;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}