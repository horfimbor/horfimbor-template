
watch-client:
    cargo watch -w client -w shared -- \
        wasm-pack build ./client \
          --target web \
          --out-dir ../server/web/template \
          --out-name index

watch-server:
    export $(grep -v '^#' .env.local | xargs) && \
        cargo watch -w server -w shared -w state -i server/web/ -i server/templates \
        -x "run -p template-server"

consume kind:
    export $(grep -v '^#' .env.local | xargs) && \
        cargo run -p template-consumer -- --consumer {{kind}}

precommit:
    cargo fmt
    cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used