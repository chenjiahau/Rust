use actix_web::{HttpResponse, http::StatusCode};

pub struct ApiResponse<T> {
    pub status_code: StatusCode,
    pub body: Option<T>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::OK,
            body: Some(data),
        }
    }

    pub fn not_found(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::NOT_FOUND,
            body: Some(data),
        }
    }

    pub fn unauthorized(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::UNAUTHORIZED,
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
