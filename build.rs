use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");

    let proto_dir = PathBuf::from("proto");
    let mut proto_files = vec![];

    // Build all .proto files
    for entry in fs::read_dir(&proto_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("proto") {
            proto_files.push(path);
        }
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut config = prost_build::Config::new();

    config.out_dir(&out_dir);
    config.compile_protos(
        &proto_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        &["proto"],
    )?;

    Ok(())
}
