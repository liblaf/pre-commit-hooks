use std::{borrow::Cow, process::Stdio};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::log::LogResult;

pub async fn prettier_yaml(content: &str) -> Cow<str> {
    prettier(content, "yaml").await
}

pub async fn prettier<'a>(content: &'a str, parser: &str) -> Cow<'a, str> {
    match prettier_unsafe(content, parser).await {
        Ok(content) => Cow::Owned(content),
        Err(_) => Cow::Borrowed(content),
    }
}

pub async fn prettier_unsafe(content: &str, parser: &str) -> anyhow::Result<String> {
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
        .write_all(content.as_bytes())
        .await
        .log()?;
    let mut content = String::new();
    child
        .stdout
        .unwrap()
        .read_to_string(&mut content)
        .await
        .log()?;
    Ok(content)
}
