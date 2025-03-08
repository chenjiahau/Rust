use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RoleModel {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Serialize)]
pub struct RoleResponseModel {
    pub roles: Vec<RoleModel>,
}
