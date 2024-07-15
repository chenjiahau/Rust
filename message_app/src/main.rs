#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use sqlx::{Pool, Postgres};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct User {
    #[serde(skip_serializing_if = "Option::is_none")] // Optional field
    id: Option<i32>,
    name: String,
    email: String,
}

#[get("/users")]
async fn get_users(db: &rocket::State<Pool<Postgres>>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(db.inner())
        .await
        .unwrap_or_else(|_| vec![]);

    Json(users)
}

#[post("/users", data = "<user>")]
async fn create_user(user: Json<User>, db: &rocket::State<Pool<Postgres>>) -> Json<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(db.inner())
    .await
    .unwrap_or_else(|_| User {
        id: None,
        name: "".to_string(),
        email: "".to_string(),
    });

    Json(user)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Load .env

    // Get the DATABASE_URL from environment variables
    rocket::build()
        .attach(AdHoc::try_on_ignite("Database Pool", |rocket| async {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = sqlx::PgPool::connect(&database_url).await.ok();
            match pool {
                Some(pool) => Ok(rocket.manage(pool)),
                None => Err(rocket),
            }
        }))
        .mount("/api", routes![get_users])
        .mount("/api", routes![create_user])
}
