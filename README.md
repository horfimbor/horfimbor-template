# gyg-template

this repository is a template to create microservices using [gyg-eventsource](https://github.com/galakhygame/gyg-eventsource)

## development : 

install [Rust](https://rustup.rs/)

if you don't have the db installed :
install [Docker](https://www.docker.com/)

install necessary toolchain : 
```shell
rustup toolchain install beta
rustup target add wasm32-unknown-unknown
```

install tools : 
```shell 
cargo install wasm-pack
cargo install cargo-watch
```

create the client :
```shell
cargo watch -w client -w shared -- wasm-pack build ./client --target web --out-dir ../server/web/template --out-name index
```

start the server :
```shell 
cargo watch -w server -w shared -w state -i server/web/ -i server/templates -x "run -p template-server"
```

before any commit please run the following : 

```shell
cargo fmt
cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
```
