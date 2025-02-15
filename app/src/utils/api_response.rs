use actix_web::HttpResponse;

// Enum to define the HTTP status code
pub enum HttpState {
    Ok = 200,
    Error = 500,
    NotFound = 404,
}

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
pub fn response(status: HttpState, message: &str) -> HttpResponse {
    HttpResponse::Ok().json(ResponseFormat {
        status: status as u16,
        message: message.to_string()
    })
}

// Function to return the response with data
pub fn response_with_data<T: serde::Serialize>(status: HttpState, message: &str, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(ResponseDataFormat {
        status: status as u16,
        message: message.to_string(),
        data
    })
}