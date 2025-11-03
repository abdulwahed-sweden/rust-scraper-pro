use anyhow::Result;
use log::LevelFilter;

pub fn setup_logger() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();
        
    Ok(())
}

pub fn setup_logger_with_level(level: LevelFilter) -> Result<()> {
    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();
        
    Ok(())
}

pub fn setup_test_logger() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init()?;
        
    Ok(())
}