use log::LevelFilter;
use simplelog::*;
use std::error::Error;
use std::fs::File;
use std::time::SystemTime;

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs()
        .to_string();
    let log_filename = format!("logs/vpn_client_{}.log", current_time);

    if !std::path::Path::new("logs").exists() {
        std::fs::create_dir("logs")?;
    }

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(log_filename)?,
        ),
    ])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logging() {
        setup_logging().unwrap();
        log::info!("This is a test log message");
    }
}
