use galvyn::core::stuff::schema::SchemaDateTime;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleClub {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: SchemaDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateClubRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateClubResponseError {
    pub name_already_occupied: bool,
}
