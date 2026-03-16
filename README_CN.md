# db_test

none

本项目是一个基于 `cargo-generate` 的 **项目模板**。它提供了一套基于 **[db-core-rs](https://github.com/your-username/db-core-rs)** 核心库构建的 5 层架构，使 AI 能够以极高的一致性实现业务逻辑。

## 快速开始

### 1. 生成项目

如果您还没有安装 `cargo-generate`：
```bash
cargo install cargo-generate
```

使用此模板生成新项目：
```bash
cargo generate --git https://github.com/atlas-form/db-center-template.git --name my-new-project
cd my-new-project
```

### 2. 准备环境

```bash
# 启动 PostgreSQL (需要安装 Docker)
make postgres

# 如有需要，编辑 config/services.toml
cp config/services-example.toml config/services.toml
```

### 3. 让 AI 开发功能

打开您的 AI 工具并将其引导至协议文档：

```text
请先阅读 ai_protocols/TABLE_ADDING_PROTOCOL.md 了解项目架构规范。

然后帮我实现一个新功能：
<描述您的需求>
```

---

## 架构概览

本模板遵循严格的分层架构，其核心逻辑抽象封装在外部库 **db-core-rs** 中。

```text
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  Layer 4: HTTP API (Axum)
├──────────────────────────────────────────────────────┤
│                     service                          │  Layer 3: 业务逻辑 (编排层)
├──────────────────────────────────────────────────────┤
│                      repo                            │  Layer 2: 数据访问 (隔离 SeaORM)
├──────────────────────────────────────────────────────┤
│               (外部核心库) db-core-rs                 │  Layer 1: 基础设施与共享核心
├──────────────────────────────────────────────────────┤
│                    migration                         │  Layer 0: 数据库迁移 (SeaORM)
└──────────────────────────────────────────────────────┘
```

### 各层职责

| 层级 | 组件           | 职责                               | AI 可修改 |
| ---- | -------------- | ---------------------------------- | --------- |
| 4    | `web-server`   | HTTP 路由、Handler、OpenAPI、校验  | 可以      |
| 3    | `service`      | 业务 API，编排多个 Repo            | 可以      |
| 2    | `repo`         | 单表 Service、DTO、转换 Entity     | 可以      |
| 1    | `db-core-rs`   | **外部库**: 基础 Trait, 错误定义, 连接池 | **固定**  |
| 0    | `migration`    | 数据库表结构定义                   | 可以      |

**核心原则**：`repo` 层是 **唯一** 允许直接依赖 `sea-orm` 的业务层。这能确保 ORM 的细节被封装在数据访问层内，`service` 和 `web-server` 只处理纯 Rust DTO。

---

## AI 开发执行规约

项目中包含详细的 `ai_protocols/` 指南，您应该始终将其提供给您的 AI 助手：

| 文件                                    | 用途                   |
| --------------------------------------- | ---------------------- |
| `ai_protocols/TABLE_ADDING_PROTOCOL.md` | 主执行规约 (从这里开始) |
| `ai_protocols/MIGRATION_GUIDE.md`       | 如何新增/修改数据库表  |
| `ai_protocols/REPO_GUIDE.md`            | 如何实现数据访问层     |
| `ai_protocols/SERVICE_GUIDE.md`         | 如何实现业务逻辑层     |
| `ai_protocols/WEB_SERVER_GUIDE.md`      | 如何实现 HTTP 接口层   |
| `how_to_use_ai.md`                      | 提示词模板 (复制即用)  |

---

## 开发命令

```bash
make help              # 查看所有命令
make postgres          # 启动 PostgreSQL 容器
make migrate-up        # 运行待处理的迁移
make migrate-fresh     # 重置数据库并运行所有迁移
make build             # 编译所有 crate
cargo run -p web-server # 启动 API 服务
```

Swagger UI: <http://localhost:19878/swagger-ui>

## 技术栈

| 组件     | 技术                |
| -------- | ------------------- |
| 运行时   | Tokio               |
| ORM      | SeaORM 2.0          |
| Web 框架 | Axum 0.8            |
| OpenAPI  | utoipa + Swagger UI |
| **核心库** | **db-core-rs**      |

## License

MIT or Apache-2.0
