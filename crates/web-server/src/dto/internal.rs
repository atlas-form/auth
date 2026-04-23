use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct JwtVerifyConfigResponse {
    pub public_key_pem: String,
    pub issuer: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DisplayUserIdToUuidResponse {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GetUserByIdentifierQuery {
    pub identifier: String,
}
