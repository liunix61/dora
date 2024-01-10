use dora_tracing::set_up_tracing;
use eyre::{bail, Context};
use std::path::Path;

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[clap(long)]
    pub run_dora_runtime: bool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let Args { run_dora_runtime } = clap::Parser::parse();

    if run_dora_runtime {
        return tokio::task::block_in_place(dora_daemon::run_dora_runtime);
    }
    set_up_tracing("cmake-dataflow-runner").wrap_err("failed to set up tracing")?;

    if cfg!(windows) {
        tracing::error!(
            "The c++ example does not work on Windows currently because of a linker error"
        );
        return Ok(());
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(root.join(file!()).parent().unwrap())
        .wrap_err("failed to set working dir")?;

    tokio::fs::create_dir_all("build").await?;
    let mut cmd = tokio::process::Command::new("cmake");
    cmd.arg(format!("-DDORA_ROOT_DIR={}", root.display()));
    cmd.arg("-B").arg("build");
    cmd.arg(".");
    if !cmd.status().await?.success() {
        bail!("failed to generating make file");
    }

    let mut cmd = tokio::process::Command::new("cmake");
    cmd.arg("--build").arg("build");
    if !cmd.status().await?.success() {
        bail!("failed to build a cmake-generated project binary tree");
    }

    let mut cmd = tokio::process::Command::new("cmake");
    cmd.arg("--install").arg("build");
    if !cmd.status().await?.success() {
        bail!("failed to build a cmake-generated project binary tree");
    }

    let dataflow = Path::new("dataflow.yml").to_owned();
    build_package("dora-runtime").await?;
    dora_daemon::Daemon::run_dataflow(&dataflow).await?;

    Ok(())
}

async fn build_package(package: &str) -> eyre::Result<()> {
    let cargo = std::env::var("CARGO").unwrap();
    let mut cmd = tokio::process::Command::new(&cargo);
    cmd.arg("build");
    cmd.arg("--package").arg(package);
    if !cmd.status().await?.success() {
        bail!("failed to build {package}");
    }
    Ok(())
}