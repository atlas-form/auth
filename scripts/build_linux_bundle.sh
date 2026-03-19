#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

PACKAGE_NAME="${PACKAGE_NAME:-web-server}"
TARGET="${TARGET:-x86_64-unknown-linux-gnu}"
PROFILE="${PROFILE:-release}"
CONTAINER_PLATFORM="${CONTAINER_PLATFORM:-linux/amd64}"
DIST_DIR="${DIST_DIR:-dist}"
BUNDLE_NAME="${PACKAGE_NAME}-${TARGET}"
BUNDLE_DIR="${DIST_DIR}/${BUNDLE_NAME}"
ARCHIVE_PATH="${DIST_DIR}/${BUNDLE_NAME}.tar.gz"
BIN_PATH="target/${TARGET}/${PROFILE}/${PACKAGE_NAME}"

echo "==> Building ${PACKAGE_NAME} for ${TARGET} (${PROFILE})"

if ! command -v cross >/dev/null 2>&1; then
  echo "error: cross is not installed. Run: cargo install cross"
  exit 1
fi

if ! rustup toolchain list | grep -q "nightly-${TARGET}"; then
  echo "==> Installing non-host toolchain nightly-${TARGET}"
  rustup toolchain add "nightly-${TARGET}" --profile minimal --force-non-host
fi

CROSS_CONTAINER_OPTS="--platform ${CONTAINER_PLATFORM}" \
  cross build --target "${TARGET}" --${PROFILE} -p "${PACKAGE_NAME}"

if [ ! -f "${BIN_PATH}" ]; then
  echo "error: built binary not found: ${BIN_PATH}"
  exit 1
fi

echo "==> Packaging bundle"
rm -rf "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}/bin" "${BUNDLE_DIR}/config"

cp "${BIN_PATH}" "${BUNDLE_DIR}/bin/${PACKAGE_NAME}"
chmod +x "${BUNDLE_DIR}/bin/${PACKAGE_NAME}"

if [ -f "config/services.toml" ]; then
  cp "config/services.toml" "${BUNDLE_DIR}/config/services.toml"
elif [ -f "config/services-example.toml" ]; then
  cp "config/services-example.toml" "${BUNDLE_DIR}/config/services.toml"
else
  echo "error: missing config/services.toml and config/services-example.toml"
  exit 1
fi

cat > "${BUNDLE_DIR}/run.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"
exec ./bin/web-server
EOF
chmod +x "${BUNDLE_DIR}/run.sh"

mkdir -p "${DIST_DIR}"
tar -czf "${ARCHIVE_PATH}" -C "${DIST_DIR}" "${BUNDLE_NAME}"

echo "==> Done"
echo "Bundle directory: ${BUNDLE_DIR}"
echo "Bundle archive:   ${ARCHIVE_PATH}"
