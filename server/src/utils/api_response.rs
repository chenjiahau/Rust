use actix_web::{HttpMessage, HttpResponse};
use httpstatus::StatusCode;

use crate::utils::message;

// Struct to define the response format
#[derive(serde::Serialize)]
pub struct ResponseFormat<T> {
    pub status: u16,
    pub code: u16,
    pub message: String,
    pub data: Option<T>
}

// Parse the message based on the status code
// fn parse_message(status: StatusCode, message: Option<String>) -> String {
//     let message = match message {
//         Some(msg) => msg,
//         None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
//     };

//     message
// }

// Function to return the success response
fn response_success<T: serde::Serialize>(status: StatusCode, code: u16, message: Option<String>, data: Option<T>) -> HttpResponse {
    let message = match message {
        Some(msg) => msg,
        None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
    };

    HttpResponse::Ok().json(ResponseFormat {
        status: status.as_u16(),
        code,
        message,
        data,
    })
}

// Function to return the error response
fn response_error<T: serde::Serialize>(status: StatusCode, code: u16, message: Option<String>, data: Option<T>) -> HttpResponse {
    let message = match message {
        Some(msg) => msg,
        None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
    };

    HttpResponse::InternalServerError().json(ResponseFormat {
        status: status.as_u16(),
        code,
        message,
        data,
    })
}

// Function to return the not found response
fn response_not_found<T: serde::Serialize>(status: StatusCode, code: u16, message: Option<String>, data: Option<T>) -> HttpResponse {
    let message = match message {
        Some(msg) => msg,
        None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
    };

    HttpResponse::NotFound().json(ResponseFormat {
        status: status.as_u16(),
        code,
        message,
        data,
    })
}

// Function to return the bad request response
fn response_bad_request<T: serde::Serialize>(status: StatusCode, code: u16, message: Option<String>, data: Option<T>) -> HttpResponse {
    let message = match message {
        Some(msg) => msg,
        None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
    };

    HttpResponse::BadRequest().json(ResponseFormat {
        status: status.as_u16(),
        code,
        message,
        data,
    })
}

// Function to return the response
pub fn response<T: serde::Serialize>(status: StatusCode, code: u16, message: Option<String>, data: Option<T>) -> HttpResponse {
    let number = status.as_u16();

    if number >= 200 && number < 300 {
        return response_success(status, code, message, data);
    } else if number >= 400 && number < 500 {
        return response_bad_request(status, code, message, data);
    } else if number >= 500 {
        return response_error(status, code, message, data);
    }

    response_not_found(status, code, message, data)
}

pub fn get_user_id_from_request(req: &actix_web::HttpRequest) -> Option<String> {
    let extensions = req.extensions();
    let user_id = extensions.get::<String>();
    if user_id.is_none() {
        return None;
    }

    Some(user_id.unwrap().to_string())
}