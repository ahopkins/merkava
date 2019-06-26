use fern::colors::{Color, ColoredLevelConfig};
use std::io;

pub fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Info),
        // .level_for("overly-verbose-target", log::LevelFilter::Warn),
        1 => base_config
            .level(log::LevelFilter::Info)
            .level_for("merkava", log::LevelFilter::Debug),
        // .level_for("overly-verbose-target", log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        // _3_or_more => base_config.level(log::LevelFilter::Trace),
        3 => base_config
            .level(log::LevelFilter::Info)
            .level_for("merkava", log::LevelFilter::Trace),
        _4_or_more => base_config.level(log::LevelFilter::Trace),
    };

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("program.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
                out.finish(format_args!(
                    "---\nDEBUG: {}: {}\n---",
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = chrono::Local::now().format("%H:%M:%S"),
                    target = record.target(),
                    level = colors_line.color(record.level()),
                    message = message,
                ))
            }
        })
        .chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}
