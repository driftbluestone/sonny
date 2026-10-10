use std::process::Stdio;
use rand::RngExt;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStderr, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let num: u128 = rng.random();
    let num_str = num.to_string();
    let mut child: tokio::process::Child = Command::new("cargo")
        .arg("run").arg("--bin").arg("core")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stdout");

    let mut stdout_reader = BufReader::new(stdout);
    let mut stderr_reader = BufReader::new(stderr);

    stdin.write_all(num_str.as_bytes()).await?;
    stdin.write_all(b"\n").await?;

    let proc = tokio::spawn(async move {read_stderr(&mut stderr_reader).await.unwrap()});

    loop {
        let mut buffer = String::new();
        let bytes_read = stdout_reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        if buffer == num_str {
            break;
        }
        print!("{}", buffer);
    }
    proc.abort();
    let _ = child.wait().await?;
    Ok(())
}

async fn read_stderr(stderr: &mut BufReader<ChildStderr>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut buffer = String::new();
        stderr.read_line(&mut buffer).await?;
        print!("{}", buffer);
    };
}
