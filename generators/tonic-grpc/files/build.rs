use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let proto_files = vec![
    ];

    let proto_include = vec![
        "proto"
    ];

    tonic_build::configure()
        .compile_protos(&proto_files, &proto_include)
        .unwrap_or_else(|e| panic!("Failed to compile protos , tonic: {:?}", e));

    //Inject proto files into the build script here
    Ok(())
}