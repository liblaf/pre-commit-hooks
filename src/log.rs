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
