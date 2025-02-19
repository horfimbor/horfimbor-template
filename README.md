# horfimbor-template

this repository is a template to create microservices using [horfimbor-eventsource](https://github.com/galakhygame/horfimbor-eventsource)

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
cargo install just
cargo install wasm-pack
cargo install cargo-watch
cargo install cargo-mutants
```

create the client :
```shell
just watch-client
```

start the server :
```shell 
just watch-server
```

```shell
cargo build -p template-server
```

before any commit please run the following : 

```shell
just precommit
```
