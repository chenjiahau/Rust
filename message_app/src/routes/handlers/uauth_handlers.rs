use actix_web::{post, web, Responder};
use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, Condition, ColumnTrait};
use crate::utils::{app_state::{self, AppState}, api_response};
use serde::{Serialize, Deserialize};
use sha256::digest;
use crate::utils::jwt::encode_jwt;

#[derive(Deserialize)]
struct RegisterJSON {
    name: String,
    email: String,
    password: String
}

#[derive(Deserialize)]
struct LoginJSON {
    email: String,
    password: String
}

#[derive(Serialize)]
struct RegisterResponse {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct LoginResponse {
    id: i32,
    name: String,
    email: String,
    token: String,
}

#[post("/register")]
async fn register(
    app_state: web::Data::<AppState>,
    register_json: web::Json<RegisterJSON>
) -> impl Responder {
    let register_model = entity::users::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)), // Hash the password
        is_active: Set(true),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();

    let user_data = RegisterResponse {
        name: register_model.name.clone(),
        email: register_model.email.clone(),
    };

    let response = api_response::generate_response(200, user_data);
    api_response::json_response(&response, 200)
}

#[post("/login")]
async fn login(
    app_state: web::Data::<AppState>,
    login_json: web::Json<LoginJSON>
) -> impl Responder {
    let user = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(&login_json.email))
                .add(entity::users::Column::Password.eq(digest(&login_json.password))) // Hash the password
                .add(entity::users::Column::IsActive.eq(true))
        ).one(&app_state.db).await.unwrap();

    if user.is_none() {
        return api_response::json_response(&"User not found", 404);
    }

    let mut login_data = LoginResponse {
        id: user.as_ref().unwrap().id,
        name: user.as_ref().unwrap().name.clone(),
        email: user.as_ref().unwrap().email.clone(),
        token: "".to_string(),
    };

    let token = encode_jwt(login_data.id, login_data.email.clone()).unwrap();
    login_data.token = token;

    let response = api_response::generate_response(200, login_data);
    api_response::json_response(&response, 200)
}