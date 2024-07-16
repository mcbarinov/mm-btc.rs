lint:
  cargo fmt && cargo clippy

lint-fix:
  cargo clippy --fix --lib -p proxy-checker --allow-dirty --allow-staged

test:
  cargo test

