use actix_web::{HttpResponse, http::StatusCode};

/**
 * Default response structure and implementation
 */
#[derive(serde::Serialize)]
pub struct DefaultResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

pub fn generate_response<T>(status: u16, message: &str, data: Option<T>) -> DefaultResponse<T> {
   DefaultResponse {
        status,
        message: message.to_string(),
        data
    }
}

/**
 * Default error response structure and implementation
 */
#[derive(serde::Serialize)]
pub struct DefaultErrorResponse {
    pub status: u16,
    pub message: String
}

impl DefaultErrorResponse {
    pub fn new(status: u16, message: &str) -> DefaultErrorResponse {
        DefaultErrorResponse {
            status,
            message: message.to_string()
        }
    }
}

/**
 * API response structure and implementation
 */
pub struct ApiResponse<T> {
    pub status_code: StatusCode,
    pub body: Option<T>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn unauthorized(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::UNAUTHORIZED,
            body: Some(data),
        }
    }

    pub fn ok(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::OK,
            body: Some(data),
        }
    }

    pub fn error(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::BAD_REQUEST,
            body: Some(data),
        }
    }

    pub fn not_found(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::NOT_FOUND,
            body: Some(data),
        }
    }

    pub fn to_http_response(self) -> HttpResponse {
        match self.body {
            Some(body) => HttpResponse::build(self.status_code).json(body),
            None => HttpResponse::build(self.status_code).finish(),
        }
    }
}