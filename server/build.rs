use std::process::Command;

fn main() {

    let _child = Command::new("wasm-pack")
        .arg("build")
        .arg("../client")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("../server/web/template")
        .arg("--out-name")
        .arg("index")
        .spawn()
        .expect("failed to start wasm build");
}
