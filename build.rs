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

/// Emit cargo link metadata for a directory of schema files.
/// This sets `cargo:rerun-if-changed` and `cargo:{env_var}=` for downstream crates.
fn emit_link_metadata(
    manifest_dir: &Path,
    dir: &Path,
    env_var: &str,
    extension: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let abs = manifest_dir
        .join(dir)
        .canonicalize()
        .map(|p| maybe_normalize_windows_path(&p))?;

    println!("cargo:rerun-if-changed={}", abs.display());
    println!("cargo:{}={}", env_var, abs.display());

    for entry in fs::read_dir(&abs)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some(extension) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    Ok(abs)
}

// --- Regeneration support (requires protoc + flatc) ---

#[cfg(feature = "regenerate")]
mod regen {
    use super::maybe_normalize_windows_path;
    use std::{
        env, fs,
        path::{Path, PathBuf},
    };

    /// Get the path to protoc, checking multiple sources:
    /// 1. PROTOC_EXECUTABLE environment variable
    /// 2. PROTOSOL_PROTOC environment variable
    /// 3. opt/bin/protoc relative to CARGO_MANIFEST_DIR
    fn get_protoc_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Ok(path) = env::var("PROTOC_EXECUTABLE") {
            return Ok(PathBuf::from(path));
        }
        if let Ok(path) = env::var("PROTOSOL_PROTOC") {
            return Ok(PathBuf::from(path));
        }
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
        let local_protoc = manifest_dir.join("opt").join("bin").join("protoc");
        if local_protoc.exists() {
            return Ok(local_protoc);
        }
        panic!(
            "protoc not found. Install it via ./deps.sh or set PROTOC_EXECUTABLE. \
             See .gitmodules for the pinned version."
        );
    }

    /// Get the path to flatc, checking multiple sources:
    /// 1. FLATC_EXECUTABLE environment variable
    /// 2. PROTOSOL_FLATC environment variable
    /// 3. opt/bin/flatc relative to CARGO_MANIFEST_DIR
    fn get_flatc_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Ok(path) = env::var("FLATC_EXECUTABLE") {
            return Ok(PathBuf::from(path));
        }
        if let Ok(path) = env::var("PROTOSOL_FLATC") {
            return Ok(PathBuf::from(path));
        }
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
        let local_flatc = manifest_dir.join("opt").join("bin").join("flatc");
        if local_flatc.exists() {
            return Ok(local_flatc);
        }
        panic!(
            "flatc not found. Install it via ./deps.sh or set FLATC_EXECUTABLE. \
             See .gitmodules for the pinned version."
        );
    }

    fn get_files(
        manifest_dir: &Path,
        dir: &Path,
        extension: &str,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let abs = manifest_dir
            .join(dir)
            .canonicalize()
            .map(|p| maybe_normalize_windows_path(&p))?;
        let mut files = vec![];
        for entry in fs::read_dir(&abs)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some(extension) {
                files.push(path);
            }
        }
        files.sort();
        Ok(files)
    }

    pub fn compile_protos(
        manifest_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let proto_dir = PathBuf::from("proto");
        let proto_files = get_files(manifest_dir, &proto_dir, "proto")?;
        let abs_proto_dir = manifest_dir
            .join(&proto_dir)
            .canonicalize()
            .map(|p| maybe_normalize_windows_path(&p))?;

        let protoc_path = get_protoc_path()?;
        let mut config = prost_build::Config::new();
        config.protoc_executable(protoc_path);
        config.out_dir(out_dir);
        config.compile_protos(
            &proto_files
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>(),
            &[abs_proto_dir],
        )?;

        Ok(())
    }

    pub fn compile_flatbuffers(
        manifest_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let flatbuffer_dir = PathBuf::from("flatbuffers");
        let flatbuffer_files = get_files(manifest_dir, &flatbuffer_dir, "fbs")?;
        let abs_flatbuffer_dir = manifest_dir
            .join(&flatbuffer_dir)
            .canonicalize()
            .map(|p| maybe_normalize_windows_path(&p))?;

        let flatc_path = get_flatc_path()?;
        let flatc = flatc_rust::Flatc::from_path(&flatc_path);
        flatc.check()?;
        flatc.run(flatc_rust::Args {
            lang: "rust",
            inputs: flatbuffer_files
                .iter()
                .map(|p| p.as_path())
                .collect::<Vec<_>>()
                .as_slice(),
            out_dir,
            includes: &[abs_flatbuffer_dir.as_path()],
            extra: &["--gen-object-api", "--gen-compare"],
            ..Default::default()
        })?;

        Ok(())
    }

    /// Remove existing .rs files from src/generated/ to avoid stale outputs.
    pub fn clean_generated(generated_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(generated_dir)?;
        for entry in fs::read_dir(generated_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                fs::remove_file(&path)?;
            }
        }
        Ok(())
    }

    /// Parse .gitmodules and print pinned versions as cargo warnings for traceability.
    pub fn print_pinned_versions(manifest_dir: &Path) {
        let gitmodules_path = manifest_dir.join(".gitmodules");
        if let Ok(content) = fs::read_to_string(&gitmodules_path) {
            let mut current_name = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("[submodule ") {
                    current_name = trimmed
                        .trim_start_matches("[submodule \"")
                        .trim_end_matches("\"]")
                        .to_string();
                } else if trimmed.starts_with("branch = ") {
                    let branch = trimmed.trim_start_matches("branch = ");
                    println!("cargo:warning=protosol: {current_name} branch hint {branch} (from .gitmodules)");
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // Always emit link metadata for downstream crates
    emit_link_metadata(&manifest_dir, Path::new("proto"), "PROTO_DIR", "proto")?;
    emit_link_metadata(
        &manifest_dir,
        Path::new("flatbuffers"),
        "FLATBUFFERS_DIR",
        "fbs",
    )?;

    // Regeneration: rebuild from proto/fbs sources directly into src/generated/
    #[cfg(feature = "regenerate")]
    {
        let generated_dir = manifest_dir.join("src").join("generated");
        regen::print_pinned_versions(&manifest_dir);
        regen::clean_generated(&generated_dir)?;
        regen::compile_protos(&manifest_dir, &generated_dir)?;
        regen::compile_flatbuffers(&manifest_dir, &generated_dir)?;
        println!(
            "cargo:warning=protosol: regenerated src/generated/ from proto/flatbuffers sources"
        );
    }

    Ok(())
}
