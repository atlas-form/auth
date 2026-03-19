#!/usr/bin/env bash

set -euo pipefail

KEY_DIR="${1:-config/key}"

mkdir -p "${KEY_DIR}"

access_private="${KEY_DIR}/access_private_key.pem"
access_public="${KEY_DIR}/access_public_key.pem"
refresh_private="${KEY_DIR}/refresh_private_key.pem"
refresh_public="${KEY_DIR}/refresh_public_key.pem"

find_usable_openssl() {
  local candidates=()

  if command -v openssl >/dev/null 2>&1; then
    candidates+=("$(command -v openssl)")
  fi
  [ -x "/opt/homebrew/opt/openssl@3/bin/openssl" ] && candidates+=("/opt/homebrew/opt/openssl@3/bin/openssl")
  [ -x "/usr/local/opt/openssl@3/bin/openssl" ] && candidates+=("/usr/local/opt/openssl@3/bin/openssl")
  [ -x "/opt/homebrew/bin/openssl" ] && candidates+=("/opt/homebrew/bin/openssl")
  [ -x "/usr/local/bin/openssl" ] && candidates+=("/usr/local/bin/openssl")

  local bin
  local tmp
  for bin in "${candidates[@]}"; do
    tmp="$(mktemp)"
    if "${bin}" genpkey -algorithm ED25519 -out "${tmp}" >/dev/null 2>&1; then
      rm -f "${tmp}"
      echo "${bin}"
      return 0
    fi
    rm -f "${tmp}"
  done
  return 1
}

OPENSSL_BIN="$(find_usable_openssl || true)"
if [ -z "${OPENSSL_BIN}" ]; then
  echo "error: no OpenSSL binary with ED25519 support found."
  echo "hint: macOS can install via: brew install openssl@3"
  exit 1
fi

rm -f "${access_private}" "${access_public}" "${refresh_private}" "${refresh_public}"
"${OPENSSL_BIN}" genpkey -algorithm ED25519 -out "${access_private}"
"${OPENSSL_BIN}" pkey -in "${access_private}" -pubout -out "${access_public}"
"${OPENSSL_BIN}" genpkey -algorithm ED25519 -out "${refresh_private}"
"${OPENSSL_BIN}" pkey -in "${refresh_private}" -pubout -out "${refresh_public}"

chmod 600 "${access_private}" "${refresh_private}"
chmod 644 "${access_public}" "${refresh_public}"

echo "Generated Ed25519 key pairs in ${KEY_DIR}:"
echo "OpenSSL binary: ${OPENSSL_BIN}"
echo "  - access_private_key.pem"
echo "  - access_public_key.pem"
echo "  - refresh_private_key.pem"
echo "  - refresh_public_key.pem"
