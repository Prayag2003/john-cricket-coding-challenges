use log::LevelFilter;

pub fn init_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file("shell.log")?)
        .apply()?;

    Ok(())
} 