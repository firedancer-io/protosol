use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = PathBuf::from("proto");
    let abs = env::current_dir()
        .expect("cwd")
        .join(&proto_dir)
        .canonicalize()
        .expect("canonicalize proto dir");

    println!("cargo:rerun-if-changed={}", abs.display());
    println!("cargo:PROTO_DIR={}", abs.display());

    let mut proto_files = vec![];
    for entry in fs::read_dir(&abs)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("proto") {
            println!("cargo:rerun-if-changed={}", path.display());
            proto_files.push(path);
        }
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let mut config = prost_build::Config::new();
    config.out_dir(&out_dir);
    config.compile_protos(
        &proto_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        &[abs],
    )?;

    Ok(())
}
