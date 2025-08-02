use chrono::{NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use diesel::prelude::*;
#[cfg(feature = "server")]
use super::schema::*;
use uuid::Uuid;
#[cfg(feature = "server")]
use diesel::deserialize::FromSql;
#[cfg(feature = "server")]
use diesel::serialize::{ToSql, Output};
#[cfg(feature = "server")]
use diesel::sql_types::Text;
#[cfg(feature = "server")]
use diesel::sqlite::Sqlite;
#[cfg(feature = "server")]
use diesel::{AsExpression, FromSqlRow, backend::Backend};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "server", diesel(sql_type = Text))]
pub struct Id(pub Uuid);

#[cfg(feature = "server")]
impl ToSql<Text, Sqlite> for Id {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

#[cfg(feature = "server")]
impl FromSql<Text, Sqlite> for Id {
    fn from_sql(mut bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.read_blob())?;
        let uuid = Uuid::parse_str(s)?;
        Ok(Id(uuid))
    }
}

#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", diesel(table_name = tasks))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Task {
    pub id: Id,
    pub title: String,
    pub important: bool,
    pub urgent: bool,
    pub content: Option<String>,
    pub completed: bool,
    pub role_id: Option<Id>,
    pub backlog_id: Option<Id>,
    pub scheduled_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TaskFilter {
    pub scheduled_date: Option<NaiveDate>,
    pub backlog_id: Option<Id>,
}

#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", diesel(table_name = backlogs))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Backlog {
    pub id: Id,
    pub name: String,
}

#[cfg_attr(feature = "server", derive(Queryable, Insertable, Selectable))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", diesel(table_name = roles))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Role {
    pub id: Id,
    pub name: String,
}
