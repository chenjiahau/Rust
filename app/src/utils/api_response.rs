use actix_web::HttpResponse;
use httpstatus::StatusCode;

// Struct to define the response format
#[derive(serde::Serialize)]
pub struct ResponseFormat {
    pub status: u16,
    pub message: String
}

// Struct to define the response format with data
#[derive(serde::Serialize)]
pub struct ResponseDataFormat<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

// Function to return the response
pub fn response(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::Ok().json(ResponseFormat {
        status: status.as_u16(),
        message: message.to_string()
    })
}

// Function to return the response with data
pub fn response_with_data<T: serde::Serialize>(status: StatusCode, message: &str, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(ResponseDataFormat {
        status: status.as_u16(),
        message: message.to_string(),
        data
    })
}

// Function to return the error response
pub fn response_error(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ResponseFormat {
        status: status.as_u16(),
        message: message.to_string()
    })
}

// Function to return the not found response
pub fn response_not_found(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ResponseFormat {
        status: status.as_u16(),
        message: message.to_string()
    })
}

// Function to return the bad request response
pub fn response_bad_request(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ResponseFormat {
        status: status.as_u16(),
        message: message.to_string()
    })
}