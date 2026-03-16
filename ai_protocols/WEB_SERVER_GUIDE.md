# Web Server 层开发协议 (AI 专用)

> 本文档是 **执行规约**，AI 必须严格按步骤在 `crates/web-server` 中添加或修改 HTTP 接口。
> `web-server` 负责 HTTP 请求响应、参数校验、身份认证、OpenAPI 文档生成。它直接调用 `service` 层的业务 API。

---

## 一、核心原则

1. **绝对禁止数据库操作**：此层绝不允许出现 `sea-orm` 或任何数据库特有类型（比如 `DbErr`、`Model`）。所有业务逻辑都应代理给 `service` 层的 API。
2. **OpenAPI 一等公民**：所有接口和 DTO 必须使用 `utoipa` 编写注释，保证 `/api-docs/openapi.json` 能生成完整的文档。
3. **分层错误处理**：业务层的 `Error`（如 NotFound, ValidationError）在这一层需要被转换为合适的 HTTP 状态码并封装成标准响应。
4. **强类型约束**：请求参数需通过 `validator` 和 `serde` 在进入 Handler 前完成校验。

---

## 二、标准开发流程

当需要新增一个业务模块（例如 `user_biz`）的 HTTP 接口时，请遵循以下步骤：

### Step 1: 编写 Web DTO (`dto/xxx.rs`)

在 `crates/web-server/src/dto/user_biz.rs` 中定义 HTTP 专用的请求和响应结构，并加上 `utoipa` 和 `validator` 注解：

```rust
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};
use validator::Validate;

/// 创建用户请求参数
#[derive(Debug, Deserialize, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
}

/// HTTP 通用响应数据体
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
}
```

### Step 2: 编写 Handler (`handlers/xxx.rs`)

在 `crates/web-server/src/handlers/user_biz.rs` 中编写处理函数。通过依赖注入（如 `get_default_ctx` 或提取器）获取所需的上下文，调用 `service` 层，然后返回 `toolcraft_axum_kit::ResponseResult`。

```rust
use axum::{extract::Query, Json};
use toolcraft_axum_kit::{CommonResponse, IntoCommonResponse, ResponseResult};
// 引入下层业务服务
use service::api::user_biz::UserBizApi;
// 引入全局静态依赖
use crate::{dto::user_biz::*, error::Error, statics::db_manager::get_default_ctx};

#[utoipa::path(
    post,
    path = "/users",
    tag = "用户业务 (User Biz)",
    params(CreateUserRequest),
    responses(
        (status = 200, description = "成功创建", body = CommonResponse<UserResponse>),
        (status = 400, description = "参数校验失败"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn create_user_handler(
    Query(req): Query<CreateUserRequest>,
) -> ResponseResult<UserResponse> {
    // 1. 初始化 service 层的业务 API
    let api = UserBizApi::new(get_default_ctx());

    // 2. 将 Web 层 DTO 转为 Service 层 DTO 并调用业务层
    let biz_req = service::dto::user_biz::CreateUserRequest {
        username: req.username,
        email: req.email,
    };
    
    // 3. 调用业务逻辑并进行错误转换
    let biz_resp = api.create_user(biz_req)
        .await
        .map_err(|e| Error::Custom(e.to_string()))?; // 注意: 实际项目中建议将错误映射为适当的 HTTP Error

    // 4. 将业务结果映射为 HTTP 层响应
    let resp = UserResponse {
        id: biz_resp.id,
        username: biz_resp.username,
        email: biz_resp.email,
    };

    Ok(resp.into_common_response().to_json())
}
```

### Step 3: 注册 Routes 与 OpenAPI (`routes/xxx.rs`)

在 `crates/web-server/src/routes/user_biz.rs` 中定义子路由结构。使用 `utoipa_axum::router::OpenApiRouter` 以自动收集路由和文档：

```rust
use axum::routing::post;
use utoipa_axum::router::OpenApiRouter;

use crate::handlers::user_biz::create_user_handler;

pub fn user_biz_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/users", post(create_user_handler))
}
```

### Step 4: 挂载到主路由 (`routes/mod.rs`)

在 `crates/web-server/src/routes/mod.rs` 中将子路由挂载上去。

```rust
mod user_biz; // 添加这个模块

use std::sync::Arc;
use axum::{Extension, Router};
use toolcraft_axum_kit::middleware::cors::create_cors;
use toolcraft_jwt::Jwt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/user-biz", api = user_biz::user_biz_routes), // 注册 API 模块的文档
    ),
)]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();
    
    // 这里使用 utoipa 收集出来的 OpenApi 定义
    let doc = ApiDoc::openapi();

    Router::new()
        // 挂载实际的路由模块
        .nest("/user-biz", user_biz::user_biz_routes())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
```

### Step 5: 导出模块

别忘了在 `crates/web-server/src/dto/mod.rs` 和 `crates/web-server/src/handlers/mod.rs` 中声明对应的新模块：
```rust
pub mod user_biz;
```

---

## 三、自检清单

- [ ] 是否确保此层没有引入 `sea-orm` 或调用任何原 `repo` 层的直接方法？
- [ ] 所有新创建的 DTO 和 Handler 是否都已经添加了完整的 `#[utoipa::...]` 注解？
- [ ] 在 `routes/mod.rs` 中是否**既挂载了实际路由 (.nest)**，**又注册了文档 (.openapi(nest(...)))**？
- [ ] 数据结构的参数验证是否完整使用了 `validator` 标签？
