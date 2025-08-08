use std::collections::HashMap;

pub fn get_common_messages() -> HashMap<&'static i32, &'static str> {
  let mut messages = HashMap::new();

  /*
   * HTTP status codes
   */
  messages.insert(&200, "Success");
  messages.insert(&201, "Created");
  messages.insert(&400, "Bad Request");
  messages.insert(&401, "Unauthorized");
  messages.insert(&403, "Forbidden");
  messages.insert(&404, "Not Found");
  messages.insert(&409, "Conflict");
  messages.insert(&500, "Internal Server Error");

  messages
}

pub enum SuccessMessage {
  // Common messages
  Success,
  CreatedSuccessfully,
}

impl SuccessMessage {
  pub fn to_string(&self) -> String {
    match self {
      // Common messages
      SuccessMessage::Success => "Success".to_string(),
      SuccessMessage::CreatedSuccessfully => "Created Successfully".to_string(),
    }
  }

  pub fn to_code(&self) -> u16{
    match self {
      // Common messages
      SuccessMessage::Success => 1000,
      SuccessMessage::CreatedSuccessfully => 1001,
    }
  }
}

pub enum ErrorMessage {
  // HTTP status codes
  BadRequest,
  NotFound,
  Unauthorized,
  Forbidden,
  Conflict,
  InternalServerError,
  // Common messages
  InvalidRequest,
  SomethingWentWrong,
  // Unauth messages
  InvalidEmail,
  PasswordTooShort,
  PasswordTooLong,
  PasswordMismatch,
  EmailAlreadyRegistered,
  InvalidCredentials,
  // User messages
  UserNotFound,
  UserPasswordMismatch,
  UserAvatarFileSizeLimitExceeded,
  UserAvatarFileTypeNotAllowed,
  UserAvatarFileFailedToUpload,
  // Spending category messages
  SpendingCategoryNotFound,
  SpendingCategoryInvalidOrder,
  SpendingCategoryAlreadyExists,
  SpendingCategoryLimitReached,
  SpendingCategoryNotAllowedToDelete,
  // Place messages
  PlaceNotFound,
  PlaceAlreadyExists,
  // Consumption messages
  ConsumptionNotFound,
  // Setting messages
  SettingNotFound,
  // Monthly budget messages
  MonthlyBudgetNotFound,
  MonthlyBudgetNotDeleted,
  MonthlyBudgetNotCreated,
}

impl ErrorMessage {
  pub fn to_string(&self) -> String {
    match self {
      // HTTP status codes
      ErrorMessage::BadRequest => "Bad Request".to_string(),
      ErrorMessage::NotFound => "Not Found".to_string(),
      ErrorMessage::Unauthorized => "Unauthorized".to_string(),
      ErrorMessage::Forbidden => "Forbidden".to_string(),
      ErrorMessage::Conflict => "Conflict".to_string(),
      ErrorMessage::InternalServerError => "Internal Server Error".to_string(),
      // Common messages
      ErrorMessage::InvalidRequest => "Invalid Request".to_string(),
      ErrorMessage::SomethingWentWrong => "Something Went Wrong".to_string(),
      // Unauth messages
      ErrorMessage::InvalidEmail => "Invalid Email".to_string(),
      ErrorMessage::PasswordTooShort => "Password Too Short".to_string(),
      ErrorMessage::PasswordTooLong => "Password Too Long".to_string(),
      ErrorMessage::PasswordMismatch => "Password Mismatch".to_string(),
      ErrorMessage::EmailAlreadyRegistered => "Email Already Registered".to_string(),
      ErrorMessage::InvalidCredentials => "Invalid Credentials".to_string(),
      // User messages
      ErrorMessage::UserNotFound => "User Not Found".to_string(),
      ErrorMessage::UserPasswordMismatch => "User Password Mismatch".to_string(),
      ErrorMessage::UserAvatarFileSizeLimitExceeded => "User Avatar File Size Limit Exceeded".to_string(),
      ErrorMessage::UserAvatarFileTypeNotAllowed => "User Avatar File Type Not Allowed".to_string(),
      ErrorMessage::UserAvatarFileFailedToUpload => "User Avatar File Failed To Upload".to_string(),
      // Spending category messages
      ErrorMessage::SpendingCategoryNotFound => "Spending Category Not Found".to_string(),
      ErrorMessage::SpendingCategoryInvalidOrder => "Spending Category Invalid Order".to_string(),
      ErrorMessage::SpendingCategoryAlreadyExists => "Spending Category Already Exists".to_string(),
      ErrorMessage::SpendingCategoryLimitReached => "Spending Category Limit Reached".to_string(),
      ErrorMessage::SpendingCategoryNotAllowedToDelete => "Spending Category Not Allowed To Delete".to_string(),
      // Place messages
      ErrorMessage::PlaceNotFound => "Place Not Found".to_string(),
      ErrorMessage::PlaceAlreadyExists => "Place Already Exists".to_string(),
      // Consumption messages
      ErrorMessage::ConsumptionNotFound => "Consumption Not Found".to_string(),
      // Setting messages
      ErrorMessage::SettingNotFound => "Settings Not Found".to_string(),
      // Monthly budget messages
      ErrorMessage::MonthlyBudgetNotFound => "Monthly Budget Not Found".to_string(),
      ErrorMessage::MonthlyBudgetNotDeleted => "Monthly Budget Not Deleted".to_string(),
      ErrorMessage::MonthlyBudgetNotCreated => "Monthly Budget Not Created".to_string(),
    }
  }

  pub fn to_code(&self) -> u16{
    match self {
      // HTTP status codes
      ErrorMessage::BadRequest => 400,
      ErrorMessage::Unauthorized => 401,
      ErrorMessage::Forbidden => 403,
      ErrorMessage::NotFound => 404,
      ErrorMessage::Conflict => 409,
      ErrorMessage::InternalServerError => 500,
      // Common messages
      ErrorMessage::InvalidRequest => 1000,
      ErrorMessage::SomethingWentWrong => 1001,
      // Unauth messages
      ErrorMessage::InvalidEmail => 2000,
      ErrorMessage::PasswordTooShort => 2001,
      ErrorMessage::PasswordTooLong => 2002,
      ErrorMessage::PasswordMismatch => 2003,
      ErrorMessage::EmailAlreadyRegistered => 2004,
      ErrorMessage::InvalidCredentials => 2005,
      // User messages
      ErrorMessage::UserNotFound => 3000,
      ErrorMessage::UserPasswordMismatch => 3001,
      ErrorMessage::UserAvatarFileSizeLimitExceeded => 3002,
      ErrorMessage::UserAvatarFileTypeNotAllowed => 3003,
      ErrorMessage::UserAvatarFileFailedToUpload => 3004,
      // Spending category messages
      ErrorMessage::SpendingCategoryNotFound => 4000,
      ErrorMessage::SpendingCategoryInvalidOrder => 4001,
      ErrorMessage::SpendingCategoryAlreadyExists => 4002,
      ErrorMessage::SpendingCategoryLimitReached => 4003,
      ErrorMessage::SpendingCategoryNotAllowedToDelete => 4004,
      // Place messages
      ErrorMessage::PlaceNotFound => 5000,
      ErrorMessage::PlaceAlreadyExists => 5001,
      // Consumption messages
      ErrorMessage::ConsumptionNotFound => 6000,
      // Setting messages
      ErrorMessage::SettingNotFound => 7000,
      // Monthly budget messages
      ErrorMessage::MonthlyBudgetNotFound => 8000,
      ErrorMessage::MonthlyBudgetNotDeleted => 8001,
      ErrorMessage::MonthlyBudgetNotCreated => 8002,
    }
  }
}