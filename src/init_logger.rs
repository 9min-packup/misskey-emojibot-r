use anstyle;
use log::Level;
use env_logger::Builder;
use std::io::Write;

pub fn init () {
    Builder::from_default_env()
    .format(|buf, record| {
        let color = match record.level() {
            Level::Trace => anstyle::AnsiColor::White,
            Level::Debug => anstyle::AnsiColor::Cyan,
            Level::Info => anstyle::AnsiColor::Green,
            Level::Warn => anstyle::AnsiColor::Yellow,
            Level::Error => anstyle::AnsiColor::Red,
        };
        let style = anstyle::Style::new().fg_color(Some(color.into()));

        writeln!(
            buf,
            "{}[{} {} {}] {}{:#}",
            style,
            buf.timestamp(),
            record.level(),
            record.target(),
            record.args(),
            style,
        )
    })
    .init();
}