extern crate flatc_rust;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Normalize path only on Windows, else just return unchanged.
fn maybe_normalize_windows_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        const EXTENDED_PREFIX: &str = r"\\?\";
        let s = path.display().to_string();
        if s.starts_with(EXTENDED_PREFIX) {
            PathBuf::from(&s[EXTENDED_PREFIX.len()..])
        } else {
            path.to_path_buf()
        }
    }
    #[cfg(not(windows))]
    {
        path.to_path_buf()
    }
}

fn monitor_and_get_files(
    dir: &PathBuf,
    env_var: &str,
    extension: &str,
) -> Result<(Vec<PathBuf>, PathBuf), Box<dyn std::error::Error>> {
    let abs = env::current_dir()
        .expect("cwd")
        .join(dir)
        .canonicalize()
        .map(|p| maybe_normalize_windows_path(&p))
        .expect("canonicalize dir");

    println!("cargo:rerun-if-changed={}", abs.display());
    println!("cargo:{}={}", env_var, abs.display());

    let mut files = vec![];
    for entry in fs::read_dir(&abs)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some(extension) {
            println!("cargo:rerun-if-changed={}", path.display());
            files.push(path);
        }
    }

    Ok((files, abs))
}

fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = PathBuf::from("proto");
    let (proto_files, abs) = monitor_and_get_files(&proto_dir, "PROTO_DIR", "proto")?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let mut config = prost_build::Config::new();
    config.protoc_executable(PathBuf::from("opt/bin/protoc"));
    config.out_dir(&out_dir);
    config.compile_protos(
        &proto_files
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        &[abs],
    )?;

    Ok(())
}

fn compile_flatbuffers() -> Result<(), Box<dyn std::error::Error>> {
    let flatbuffer_dir = PathBuf::from("flatbuffers");
    let (flatbuffer_files, _) = monitor_and_get_files(&flatbuffer_dir, "FLATBUFFERS_DIR", "fbs")?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Use custom flatc from ./opt/bin/flatc relative to this build.rs
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let flatc_path = manifest_dir.join("opt").join("bin").join("flatc");

    let flatc = flatc_rust::Flatc::from_path(&flatc_path);
    flatc.check()?;
    flatc.run(flatc_rust::Args {
        lang: "rust",
        inputs: flatbuffer_files
            .iter()
            .map(|p| p.as_path())
            .collect::<Vec<_>>()
            .as_slice(),
        out_dir: out_dir.as_path(),
        includes: &[flatbuffer_dir.as_path()],
        extra: &["--gen-object-api", "--gen-compare"],
        ..Default::default()
    })?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos()?;
    compile_flatbuffers()?;
    Ok(())
}
