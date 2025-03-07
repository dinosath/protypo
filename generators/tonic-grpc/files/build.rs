use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/healthcheck.proto")?;

    //Inject proto files into the build script here
    Ok(())
}
