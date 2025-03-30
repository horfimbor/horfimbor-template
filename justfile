set shell := ["bash", "-uc"]
set dotenv-load

dc-start *SRV:
    docker compose up -d --build --force-recreate {{SRV}}

dc-stop:
    docker compose down

dc-reset:
    docker compose down -v
    just dc-start


alias ff := open
open:
    firefox $APP_HOST

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

test-mutation:
    cargo test
    cargo mutants -p template-state