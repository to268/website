use std::{env, str::FromStr};

#[inline(always)]
fn get_default_log_level() -> log::Level {
    if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    }
}

pub fn init() {
    let level = match env::var_os("RUST_LOG") {
        Some(level) => match level.to_str() {
            Some(level) => {
                log::Level::from_str(level).unwrap_or(get_default_log_level())
            }
            None => get_default_log_level(),
        },
        None => get_default_log_level(),
    };

    simple_logger::init_with_level(level).expect("couldn't initialize logging");
}
