#[cfg(feature = "logging")]
use env_logger;
pub use log::debug;
pub use log::info;

pub fn init() {
    env_logger::init();
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info {
    (*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug {
    (*) => {};
}

