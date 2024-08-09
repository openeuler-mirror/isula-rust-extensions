use ttrpc_codegen::{Codegen, Customize, ProtobufCustomize};

fn main() {
    let protobuf_customized = ProtobufCustomize::default().gen_mod_rs(false);

    Codegen::new()
        .out_dir("src/protocols")
        .inputs(&[
            "src/protocols/protos/nri.proto",
        ])
        .include("src/protocols/protos")
        .rust_protobuf() // also generate protobuf messages, not just services
        .customize(Customize {
            ..Default::default()
        })
        .rust_protobuf_customize(protobuf_customized.clone())
        .run()
        .expect("Codegen failed.");
}
