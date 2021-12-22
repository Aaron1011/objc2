set -euxo pipefail

git fetch origin 242e218400c16071c42a21024229d629a511beaa
cargo test --no-fail-fast --no-default-features
git checkout 242e218400c16071c42a21024229d629a511beaa
cargo test --no-fail-fast --no-default-features
