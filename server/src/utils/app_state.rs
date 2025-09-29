use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub name: String,
    pub version: String,
    pub db: DatabaseConnection,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_sender: String,
}
