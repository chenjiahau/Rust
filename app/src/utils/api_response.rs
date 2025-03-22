use actix_web::HttpResponse;
use httpstatus::StatusCode;

use crate::utils::message;

// Struct to define the response format
#[derive(serde::Serialize)]
pub struct ResponseFormat<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

// Parse the message based on the status code
fn parse_message(status: StatusCode, message: Option<String>) -> String {
    let message = match message {
        Some(msg) => msg,
        None => message::get_common_messages().get(&(status.as_u16() as i32)).unwrap().to_string(),
    };

    message
}

// Function to return the success response
fn response_success<T: serde::Serialize>(status: StatusCode, message: Option<String>, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(ResponseFormat {
        status: status.as_u16(),
        message: parse_message(status.clone(), message),
        data,
    })
}

// Function to return the error response
fn response_error<T: serde::Serialize>(status: StatusCode, message: Option<String>, data: Option<T>) -> HttpResponse {
    HttpResponse::InternalServerError().json(ResponseFormat {
        status: status.as_u16(),
        message: parse_message(status.clone(), message),
        data,
    })
}

// Function to return the not found response
fn response_not_found<T: serde::Serialize>(status: StatusCode, message: Option<String>, data: Option<T>) -> HttpResponse {
    HttpResponse::NotFound().json(ResponseFormat {
        status: status.as_u16(),
        message: parse_message(status.clone(), message),
        data,
    })
}

// Function to return the bad request response
fn response_bad_request<T: serde::Serialize>(status: StatusCode, message: Option<String>, data: Option<T>) -> HttpResponse {
    HttpResponse::BadRequest().json(ResponseFormat {
        status: status.as_u16(),
        message: parse_message(status.clone(), message),
        data,
    })
}

// Function to return the response
pub fn response<T: serde::Serialize>(status: StatusCode, message: Option<String>, data: Option<T>) -> HttpResponse {
    let number = status.as_u16();

    if number >= 200 && number < 300 {
        return response_success(status, message, data);
    } else if number >= 400 && number < 500 {
        return response_bad_request(status, message, data);
    } else if number >= 500 {
        return response_error(status, message, data);
    }

    response_not_found(status, message, data)
}