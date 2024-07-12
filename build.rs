use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Result};

fn tag_exists(name: &str, tag: &str) -> Result<bool> {
    let cmd = Command::new("docker")
        .arg("image")
        .arg("inspect")
        .arg(format!("{name}:{tag}"))
        .output()?;

    Ok(cmd.status.success())
}

fn build_yral_metadata(cwd: &str, meta_dir: &Path) -> Result<()> {
    let repo = gix::open(format!("{cwd}/yral-metadata"))?;
    let id = repo.head_id()?.to_hex().to_string();
    let image_name = "yral-metadata";

    let mut meta_file = meta_dir.to_path_buf();
    meta_file.push("yral_metadata");
    meta_file.set_extension("rs");
    let meta_file_content = format!(
        r#"
const NAME: &str = "{image_name}";
const TAG: &str = "{id}";
"#
    );
    fs::write(meta_file, meta_file_content)?;

    let exists = tag_exists(image_name, &id)?;
    if exists {
        return Ok(());
    }

    let output = Command::new("docker")
        .arg("build")
        .arg("--file")
        .arg(format!("{cwd}/yral-metadata/Dockerfile.local"))
        .arg("--force-rm")
        .arg("--tag")
        .arg(format!("yral-metadata:{id}"))
        .arg("./yral-metadata")
        .output()?;
    if !output.status.success() {
        eprintln!(
            "Failed to build yral-metadata: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        bail!("faied to build yral-metadata:latest")
    }

    eprintln!("Built yral-metadata:latest");

    Ok(())
}

fn build_yral_backend(cwd: &str, meta_dir: &Path) -> Result<()> {
    let repo = gix::open(format!("{cwd}/hot-or-not-backend-canister"))?;
    let id = repo.head_id()?.to_hex().to_string();
    let image_name = "yral-backend";
    let mut meta_file = meta_dir.to_path_buf();
    meta_file.push("yral_backend");
    meta_file.set_extension("rs");
    let meta_file_content = format!(
        r#"
const NAME: &str = "{image_name}";
const TAG: &str = "{id}";
"#
    );
    fs::write(meta_file, meta_file_content)?;

    let exists = tag_exists(image_name, &id)?;
    if exists {
        return Ok(());
    }

    let output = Command::new("docker")
        .arg("build")
        .arg("--force-rm")
        .arg("--tag")
        .arg(format!("yral-backend:{id}"))
        .arg("./hot-or-not-backend-canister")
        .output()?;
    if !output.status.success() {
        eprintln!(
            "Failed to build yral-backend: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        bail!("faied to build yral-backend")
    }

    eprintln!("Built yral-backend:latest");

    Ok(())
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=.git/modules/*/packed-refs");
    println!("cargo:rerun-if-changed=.git/modules/*/HEAD");

    let out_dir = env::var("OUT_DIR")?;
    let meta_dir = PathBuf::from(&out_dir).join("meta");
    fs::create_dir_all(&meta_dir)?;

    let cwd = env::var("CARGO_MANIFEST_DIR")?;

    build_yral_metadata(&cwd, &meta_dir)?;
    build_yral_backend(&cwd, &meta_dir)?;

    Ok(())
}
