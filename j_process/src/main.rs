#![allow(dead_code)]

use std::error::Error;
use tokio::{io::AsyncWriteExt, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // run_git().await
    run_sort().await
}

async fn run_sort() -> Result<(), Box<dyn Error>> {
    let mut cmd = process::Command::new("sort");
    cmd.stdout(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let chars: &[&str] = &["c", "g", "e", "a", "b"];
    let mut child_proc = cmd.spawn()?;

    let mut stdin = child_proc.stdin.take().expect("stdin pipe");
    stdin
        .write_all(chars.join("\n").as_bytes())
        .await
        .expect("being able to write to stdin");

    drop(stdin);
    // stdin.shutdown().await?;

    let output = child_proc.wait_with_output().await?;
    println!("got output:\n{}", std::str::from_utf8(&output.stdout)?);
    println!("got error:\n{}", std::str::from_utf8(&output.stderr)?);

    Ok(())
}

async fn run_git() -> Result<(), Box<dyn Error>> {
    let mut cmd = process::Command::new("git");
    cmd.args(&["branch", "-a"])
        .stdout(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    eprintln!("executing command [{:?}]", cmd.as_std());

    let mut child_proc = cmd.spawn()?;

    let mut stdin = child_proc.stdin.take().expect("stdin pipe");
    stdin.write_all(b"").await?;

    drop(stdin);

    let output = child_proc.wait_with_output().await?;
    println!("got output:\n{}", std::str::from_utf8(&output.stdout)?);
    println!("got error:\n{}", std::str::from_utf8(&output.stderr)?);
    Ok(())
}
