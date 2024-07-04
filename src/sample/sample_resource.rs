use crate::database::{DatabaseSession, DATABASE_SESSION};
use crate::entities::samples::

pub struct SampleResource {
    name: String,
    db_session: &'static DatabaseSession,
}

impl SampleResource {

    pub fn new(name: String) -> Result<Self, String> {
        match DATABASE_SESSION.get() {
            Some(session) => Ok(Self { name, db_session: session }),
            None => Err("Database session not initialized".to_string()),
        }
        Self { name, db_session }
    }

    pub async fn save(&self) -> Result<(), String> {
        let db = &self.db_session.connection;

    }

}