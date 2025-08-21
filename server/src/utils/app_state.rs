use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub version: String,
    pub db: DatabaseConnection,
}
