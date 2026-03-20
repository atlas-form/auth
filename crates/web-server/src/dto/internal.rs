use serde::Serialize;
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
