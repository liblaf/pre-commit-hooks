use std::panic::Location;

use clap_verbosity_flag::{Level, LogLevel, Verbosity};

#[derive(Copy, Clone, Debug, Default)]
pub struct DefaultLevel;

impl LogLevel for DefaultLevel {
    fn default() -> Option<Level> {
        if shadow_rs::is_debug() {
            Some(Level::Debug)
        } else {
            Some(Level::Info)
        }
    }
}

pub trait LogInit {
    fn init(&self);
}

impl<L> LogInit for Verbosity<L>
where
    L: LogLevel,
{
    fn init(&self) {
        if let Some(level) = self.log_level() {
            let level = level.to_string().parse::<tracing::Level>().unwrap();
            if level < tracing::Level::DEBUG {
                tracing_subscriber::fmt()
                    .with_max_level(level)
                    .with_target(false)
                    .with_writer(std::io::stderr)
                    .without_time()
                    .init();
            } else {
                tracing_subscriber::fmt()
                    .with_file(true)
                    .with_line_number(true)
                    .with_max_level(level)
                    .with_writer(std::io::stderr)
                    .init();
            }
        }
    }
}

pub trait LogErr {
    #[track_caller]
    fn log(self) -> anyhow::Error;
}

impl<E> LogErr for E
where
    E: Into<anyhow::Error>,
{
    #[track_caller]
    fn log(self) -> anyhow::Error {
        let err = self.into();
        let mut message = err.to_string();
        let sources = err
            .chain()
            .skip(1)
            .enumerate()
            .map(|(i, err)| format!("{:>5}: {}", i, err))
            .collect::<Vec<_>>()
            .join("\n");
        if !sources.is_empty() {
            message += "\nCaused by:\n";
            message += sources.as_str();
            message += "\n";
        }
        let location = Location::caller();
        tracing::error!(%location, message);
        err
    }
}

pub trait LogResult<T> {
    #[track_caller]
    fn log(self) -> anyhow::Result<T>;
}

impl<T, E> LogResult<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    #[track_caller]
    fn log(self) -> anyhow::Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err.log()),
        }
    }
}
