set shell := ["bash", "-uc"]
set dotenv-load

startup:
    docker compose up -d

stop:
    docker compose down

watch-client:
    cargo watch -w client -w shared -- \
        wasm-pack build ./client \
          --target web \
          --out-dir ../server/web/template \
          --out-name index-v0-1-0

watch-server:
    cargo watch -w server -w shared -w state -i server/web/ -i server/templates \
        -x "run -p template-server service"

precommit:
    cargo fmt
    cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
    cargo test