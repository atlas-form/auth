mod auth;
mod user;

use std::sync::Arc;

use auth::{AuthApiDoc, auth_routes};
use axum::{Extension, Router};
use toolcraft_axum_kit::{Empty, middleware::cors::create_cors};
use toolcraft_jwt::Jwt;
use user::{UserApiDoc, user_routes};
use utoipa::{
    OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/auth", api = AuthApiDoc),
        (path = "/user", api = UserApiDoc),
    ),
    components(schemas(Empty))
)]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();

    let mut doc = ApiDoc::openapi();
    doc.components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "Bearer",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "Authorization",
                "Enter: Bearer <token>",
            ))),
        );

    Router::new()
        .nest("/auth", auth_routes())
        .nest("/user", user_routes())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
