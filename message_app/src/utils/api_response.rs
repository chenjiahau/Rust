use std::fmt::Display;
use actix_web::body::BoxBody;
use actix_web::http::{header, StatusCode};
use actix_web::web::BytesMut;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;

#[derive(Debug)]
pub struct ApiResponse {
    pub body: String,
    response_code: StatusCode
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
      ApiResponse {
          body,
          response_code: StatusCode::from_u16(status_code).unwrap()
      }
  }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} \n Status Code: {}", self.body, self.response_code)
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.response_code)
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(self.body.clone())
    }
}

// This trait is implemented for ApiResponse so that it can be converted into an HTTP response
impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(BytesMut::from(self.body.as_bytes()));

        match serde_json::from_str::<serde_json::Value>(&self.body) {
            Ok(_) => {
                HttpResponse::build(self.response_code)
                    .insert_header((header::CONTENT_TYPE, "application/json"))
                    .body(body)
            }
            Err(_) => {
                HttpResponse::build(self.response_code)
                    .insert_header((header::CONTENT_TYPE, "text/plain"))
                    .body(body)
            }
        }
    }
}

/**
 * This struct is used to create a default response
 */
#[derive(Debug, Serialize)]
pub struct DefaultResponse<T> {
    pub code: i32,
    pub data: T,
}

// This function is used to create a default response
pub fn generate_response<T>(code: i32, data: T) -> DefaultResponse<T> {
    DefaultResponse {
        code,
        data
    }
}