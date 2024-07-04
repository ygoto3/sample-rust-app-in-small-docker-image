use std::env;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseSession {
    pub connection: DatabaseConnection,
}

impl DatabaseSession {

    pub async fn new(db_url: String) -> Result<Self, DbErr> {
        let db_connection = Database::connect(format!("sqlite://{}", db_url)).await?;
        Ok(Self { connection: db_connection })
    }

}

pub static DATABASE_SESSION: OnceCell<DatabaseSession> = OnceCell::new();

pub async fn init_session() -> Result<(), String> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "./db.sqlite".to_string());
    let db_session = match DatabaseSession::new(db_url).await {
        Ok(session) => session,
        Err(err) => return Err(format!("Failed to connect to database: {:?}", err)),
    };
    DATABASE_SESSION.set(db_session).map_err(|_| "Failed to set database session".to_string())
}
