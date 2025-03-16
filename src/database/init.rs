use rocket_sync_db_pools::database;

#[database("sqlite_main")]
pub struct MainDbConn(diesel::SqliteConnection);