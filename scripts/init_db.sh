#!/bin/bash
set -e

# ============================
# ✅ 只需要改下面两个变量
# ============================
PG_URL_WITHOUT_DB="postgres://postgres:123456@localhost:15432"   # 不带 /db
DB_NAME="auth"                                           # 目标数据库名
# ============================

echo "🔧 Initializing database..."
echo "🔗 PG_URL_WITHOUT_DB=$PG_URL_WITHOUT_DB"
echo "📦 DB_NAME=$DB_NAME"

# 拼出目标数据库连接
TARGET_URL="${PG_URL_WITHOUT_DB}/${DB_NAME}"
ADMIN_URL="${PG_URL_WITHOUT_DB}/postgres"

# 1) 如果目标库已存在且可连接，直接退出（幂等）
if psql "$TARGET_URL" -c '\q' 2>/dev/null; then
  echo "✅ Database '$DB_NAME' already exists"
  exit 0
fi

echo "⚠️ Database '$DB_NAME' does not exist, creating..."

# 2) 用 postgres 管理库创建目标库
psql "$ADMIN_URL" -c "CREATE DATABASE \"$DB_NAME\";"

echo "🎉 Database '$DB_NAME' created successfully"
echo "✅ Target URL: $TARGET_URL"
