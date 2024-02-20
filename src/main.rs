use clap::Parser;

mod cmd;
mod log;
mod proc;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await
}
