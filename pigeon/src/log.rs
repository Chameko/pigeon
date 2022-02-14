use cfg_if::cfg_if;

pub fn setup_logger(filter: log::LevelFilter) -> Result<(), fern::InitError> {
    // Setting up logger woo
    cfg_if! {
        // Only log for the console if the feature is enabled
        if #[cfg(feature = "console_log")] {
            fern::Dispatch::new()
                // formating
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%d/%m/%Y][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                // Log file with maximum verbosity
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::max())
                        .chain(fern::log_file("output.log")?)
                )
                .level(filter)
                .chain(std::io::stdout())
                .chain(fern::Output::call(console_log::log))
                .apply()?;
            Ok(())
        } else {
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%d/%m/%Y][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                // Log file with maximum verbosity
                .chain(
                    fern::Dispatch::new()
                        .level(log::LevelFilter::max())
                        .chain(fern::log_file("output.log")?)
                )
                .level(filter)
                .chain(std::io::stdout())
                .apply()?;
            Ok(())
        }
    }
    
}