# Service 层开发协议 (AI 专用)

> 本文档是 **执行规约**，AI 必须严格按步骤在 `crates/service` 中添加或修改综合业务逻辑。
> `service` 也就是之前的 `demo-db` 层，职责为：业务 API 层，编排多个 Repo，处理业务语义，不直接操作数据库。

---

## 一、核心原则

1. **跨表编排**：`service` 层的 API 是唯一允许进行多表数据组合的地方。
2. **禁止直接访问数据库**：所有对数据的读写**必须**通过调用 `repo`（原 `pg-tables`）层中提供的单表 Service 进行。绝不能直接使用 `sea-orm` 的查询构建器直接操作数据库。
3. **业务逻辑中心**：外键校验、权限判断、数据聚合、复杂的计算逻辑都应该放在这一层。
4. **面向业务的 DTO**：返回给上层（Web HTTP 层）的 DTO 应该是具有业务语义的复合结构，而非单一的表结构。

---

## 二、标准开发流程

假设我们要实现一个获取用户及其资料概览的业务功能，涉及 `user` 和 `user_profile` 两张表。

### Step 1: 定义业务 DTO (`dto/xxx.rs`)

在 `crates/service/src/dto/user_biz.rs` 中定义业务所需的请求和响应结构。

```rust
use serde::{Deserialize, Serialize};
// 引入 repo 层的单表 DTO 进行复用（如果需要）
use repo::table::user::User;
use repo::table::user_profile::UserProfile;

/// 业务请求参数
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserOverviewRequest {
    pub user_id: i64,
}

/// 业务复合响应（聚合了多表数据）
#[derive(Debug, Clone, Serialize)]
pub struct UserOverviewResponse {
    pub user: User,
    pub profiles: Vec<UserProfile>,
    // 还可以包含业务计算得出的字段
    pub profile_count: usize,
}
```

### Step 2: 实现业务 API (`api/xxx.rs`)

在 `crates/service/src/api/user_biz.rs` 中创建 API 结构体，持有对应的 Repo Service，并实现业务方法。

```rust
use db_core_rs::{DbContext, Result, PgError};
use repo::table::user::UserService;
use repo::table::user_profile::UserProfileService;

use crate::dto::user_biz::{GetUserOverviewRequest, UserOverviewResponse};

pub struct UserBizApi {
    user_svc: UserService,
    profile_svc: UserProfileService,
}

impl UserBizApi {
    // 1. 初始化时注入依赖的 Repo Service
    pub fn new(db: DbContext) -> Self {
        Self {
            user_svc: UserService::new(db.clone()),
            profile_svc: UserProfileService::new(db),
        }
    }

    // 2. 编写业务逻辑方法
    pub async fn get_user_overview(&self, req: GetUserOverviewRequest) -> Result<UserOverviewResponse> {
        // 编排 1: 获取主表数据。如果不存在，底层 repo svc 会返回 not_found 错误
        let user = self.user_svc.get(req.user_id).await?;

        // 编排 2: 获取关联表数据
        let profiles = self.profile_svc.list_by_user(req.user_id).await?;
        
        // 业务计算
        let profile_count = profiles.len();

        // 3. 组合响应
        Ok(UserOverviewResponse { 
            user, 
            profiles,
            profile_count,
        })
    }
}
```

### Step 3: 导出模块

1. 在 `crates/service/src/dto/mod.rs` 中注册：
```rust
pub mod user_biz;
```

2. 在 `crates/service/src/api/mod.rs` 中注册：
```rust
pub mod user_biz;
```

3. 如果需要暴露给外部，也可以在 `crates/service/src/lib.rs` 中处理。

---

## 三、自检清单

- [ ] 是否在 API 中注入了 `repo` 层对应的 `XXXService`？
- [ ] 数据读写是否全部**通过 `repo` 的 Service 方法**调用完成？
- [ ] 是否避免了在 `service` 中引入 `sea_orm` 和数据库驱动的细节代码？
- [ ] 业务校验（如“如果用户状态为禁用则不允许获取数据”）是否都实现在了本层？
