use actix_web::body::BoxBody;
use actix_web::http::{header, StatusCode};
use actix_web::web::BytesMut;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

pub struct ApiResponse {
    pub body: String,
    response_code: StatusCode
}

/**
 *  Response as text
 **/
impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
      ApiResponse {
          body,
          response_code: StatusCode::from_u16(status_code).unwrap()
      }
  }
}

// This trait is implemented for ApiResponse so that it can be converted into an HTTP response
impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

/**
 *  Response as JSON
 **/
// This struct is used to create a default response
#[derive(Serialize)]
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

// This function is used to create a JSON response from a serializable object
pub fn json_response<T: Serialize>(data: &T, status_code: u16) -> HttpResponse<BoxBody> {
    match serde_json::to_string(data) {
        Ok(json_body) => HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(json_body),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize JSON"),
    }
}