use std::{borrow::Cow, process::Stdio};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::log::LogResult;

pub async fn prettier_yaml(contents: &str) -> Cow<str> {
    prettier(contents, "yaml").await
}

pub async fn prettier<'a>(contents: &'a str, parser: &str) -> Cow<'a, str> {
    match prettier_unsafe(contents, parser).await {
        Ok(contents) => Cow::Owned(contents),
        Err(_) => Cow::Borrowed(contents),
    }
}

pub async fn prettier_unsafe(contents: &str, parser: &str) -> anyhow::Result<String> {
    let mut cmd = tokio::process::Command::new("prettier");
    cmd.arg("--parser")
        .arg(parser)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    let child = cmd.spawn().log()?;
    child
        .stdin
        .unwrap()
        .write_all(contents.as_bytes())
        .await
        .log()?;
    let mut contents = String::new();
    child
        .stdout
        .unwrap()
        .read_to_string(&mut contents)
        .await
        .log()?;
    Ok(contents)
}
