use std::collections::HashMap;

pub fn get_common_messages() -> HashMap<&'static i32, &'static str> {
  let mut messages = HashMap::new();

  messages.insert(&200, "Success");
  messages.insert(&201, "Created");
  messages.insert(&400, "Bad Request");
  messages.insert(&401, "Unauthorized");
  messages.insert(&404, "Not Found");
  messages.insert(&409, "Conflict");
  messages.insert(&500, "Internal Server Error");

  messages
}