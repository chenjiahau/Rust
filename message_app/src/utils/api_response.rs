use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::BytesMut;
use actix_web::{HttpRequest, HttpResponse, Responder};

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

impl Responder for ApiResponse {
    type Body = BoxBody;

    // This function is called by Actix to convert the response into an HTTP response
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}