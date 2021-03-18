#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum LogLevel {
    Quiet,
    Error,
    Info,
}

impl LogLevel {
    pub fn write(&self, level: &Self) -> Box<dyn std::io::Write> {
        match level {
            Self::Error => Box::from(std::io::stderr()),
            _ => Box::from(std::io::stdout()),
        }
    }
}

impl Into<&'static str> for LogLevel {
    fn into(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Info => "info",
            Self::Error => "error",
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        match s {
            "quiet" => Ok(Self::Quiet),
            "info" | "all" => Ok(Self::Info),
            "error" => Ok(Self::Error),
            level => Err(format!("I don't know the log level of {:?}", level)),
        }
    }
}

#[macro_export]
macro_rules! outln {
    ($config:ident#$level:path, $($expr:expr),+) => {{
        use $crate::log::LogLevel::*;
        writeln!($config.log_level.write(&$level), $($expr),+).expect("Can't write output");
    }}
}