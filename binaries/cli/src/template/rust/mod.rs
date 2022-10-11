use eyre::{bail, Context};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn create(args: crate::CommandNew) -> eyre::Result<()> {
    let crate::CommandNew {
        kind,
        lang: _,
        name,
        path,
    } = args;

    match kind {
        crate::Kind::Operator => create_operator(name, path),
        crate::Kind::CustomNode => create_custom_node(name, path),
    }
}

fn create_operator(name: String, path: Option<PathBuf>) -> Result<(), eyre::ErrReport> {
    const CARGO_TOML: &str = include_str!("operator/Cargo-template.toml");
    const LIB_RS: &str = include_str!("operator/lib-template.rs");

    if name.contains('/') {
        bail!("operator name must not contain `/` separators");
    }
    if !name.is_ascii() {
        bail!("operator name must be ASCII");
    }

    // create directories
    let root = path.as_deref().unwrap_or_else(|| Path::new(&name));
    fs::create_dir(&root)
        .with_context(|| format!("failed to create directory `{}`", root.display()))?;
    let src = root.join("src");
    fs::create_dir(&src)
        .with_context(|| format!("failed to create directory `{}`", src.display()))?;

    let cargo_toml = CARGO_TOML.replace("___name___", &name);
    let cargo_toml_path = root.join("Cargo.toml");
    fs::write(&cargo_toml_path, &cargo_toml)
        .with_context(|| format!("failed to write `{}`", cargo_toml_path.display()))?;

    let lib_rs_path = src.join("lib.rs");
    fs::write(&lib_rs_path, LIB_RS)
        .with_context(|| format!("failed to write `{}`", lib_rs_path.display()))?;

    println!(
        "Created new Rust operator `{name}` at {}",
        Path::new(".").join(root).display()
    );

    Ok(())
}

fn create_custom_node(name: String, path: Option<PathBuf>) -> Result<(), eyre::ErrReport> {
    const CARGO_TOML: &str = include_str!("node/Cargo-template.toml");
    const MAIN_RS: &str = include_str!("node/main-template.rs");

    if name.contains('/') {
        bail!("node name must not contain `/` separators");
    }
    if !name.is_ascii() {
        bail!("node name must be ASCII");
    }

    // create directories
    let root = path.as_deref().unwrap_or_else(|| Path::new(&name));
    fs::create_dir(&root)
        .with_context(|| format!("failed to create directory `{}`", root.display()))?;
    let src = root.join("src");
    fs::create_dir(&src)
        .with_context(|| format!("failed to create directory `{}`", src.display()))?;

    let cargo_toml = CARGO_TOML.replace("___name___", &name);
    let cargo_toml_path = root.join("Cargo.toml");
    fs::write(&cargo_toml_path, &cargo_toml)
        .with_context(|| format!("failed to write `{}`", cargo_toml_path.display()))?;

    let main_rs_path = src.join("main.rs");
    fs::write(&main_rs_path, MAIN_RS)
        .with_context(|| format!("failed to write `{}`", main_rs_path.display()))?;

    println!(
        "Created new Rust custom node `{name}` at {}",
        Path::new(".").join(root).display()
    );

    Ok(())
}
