use chrono::{Datelike, Utc};
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
// use sqlx::Sqlite;
use std::str::FromStr;
use uuid::Uuid;

pub mod model;
// pub mod query;

// #[derive(Debug, thiserror::Error)]
// pub enum DbError {
//     #[error("Database error: {0}")]
//     DatabaseError(#[from] sqlx::Error),
// }

// pub type AppDatabase = Database<Sqlite>;
// pub type DatabasePool = sqlx::sqlite::SqlitePool;
// pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
// pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
// pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

// pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

// impl Database<Sqlite> {
//     pub async fn new(connection_str: &str) -> Self {
//         let pool = sqlx::sqlite::SqlitePoolOptions::new()
//             .connect(connection_str)
//             .await;
//         match pool {
//             Ok(pool) => Self(pool),
//             Err(e) => {
//                 eprintln!("Failed to connect to database: {}", e);
//                 eprintln!(
//                     "If the database has not been created, please run \n $ sqlx database setup \n"
//                 );
//                 panic!("Database connection failed");
//             }
//         }
//     }
//     pub fn get_pool(&self) -> &DatabasePool {
//         &self.0
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, Display, From)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, From)]
pub struct DbWeekId(String);

impl DbWeekId {
    pub fn new() -> DbWeekId {
        let week = Utc::now().iso_week();
        let week_id = format!("{week:?}",);
        Self(week_id)
    }
}

impl From<DbWeekId> for String {
    fn from(id: DbWeekId) -> Self {
        format!("{}", id.0)
    }
}

impl Default for DbWeekId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbWeekId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
