mod internal;
mod session;
mod user;

use std::sync::Arc;

use axum::{Extension, Router};
use internal::internal_routes;
use session::{SessionApiDoc, session_routes};
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
        (path = "/auth/session", api = SessionApiDoc),
        (path = "/auth/user", api = UserApiDoc),
    ),
    components(schemas(Empty))
)]
struct ApiDoc;

pub fn create_routes(
    jwt: Arc<Jwt>,
    jwt_verify_cfg: Arc<crate::settings::JwtVerifyConfig>,
    avatar_url_cfg: Arc<crate::settings::AvatarUrlConfig>,
) -> Router {
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

    let auth_routes = Router::new()
        .nest("/session", session_routes())
        .nest("/user", user_routes());

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/internal", internal_routes())
        .layer(Extension(jwt))
        .layer(Extension(jwt_verify_cfg))
        .layer(Extension(avatar_url_cfg))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
