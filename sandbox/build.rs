use std::fs;

fn main() {
    fs::create_dir("src/controller/client/sandbox").unwrap_or_default();
    fs::create_dir("src/controller/client/cri").unwrap_or_default();
    tonic_build::configure()
        .build_server(true)
        .include_file("mod.rs")
        .out_dir("src/controller/client/sandbox")
        .compile(&["src/controller/client/protos/sandbox.proto"], &["src/controller/client/protos"])
        .unwrap();

    tonic_build::configure()
        .build_server(true)
        .include_file("mod.rs")
        .out_dir("src/controller/client/cri")
        .compile(&["src/controller/client/protos/cri-api/api.proto"], &["src/controller/client/protos"])
        .unwrap();
}
