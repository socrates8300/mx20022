#!/usr/bin/env bash
set -euo pipefail
HOOK_DIR="$(git rev-parse --git-dir)/hooks"
cat > "$HOOK_DIR/pre-commit" << 'HOOK'
#!/usr/bin/env bash
set -euo pipefail
echo "Pre-commit: fmt + clippy + check"
cargo fmt --all -- --check || { echo "Run 'cargo fmt --all'"; exit 1; }
cargo clippy --workspace --all-features -- -D warnings || exit 1
cargo check --workspace --all-features || exit 1
echo "Pre-commit passed."
HOOK
chmod +x "$HOOK_DIR/pre-commit"
echo "Git hooks installed."
