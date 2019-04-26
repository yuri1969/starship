// Lib is present to allow for benchmarking
#[macro_use]
extern crate log;

#[macro_use]
mod logging;

extern crate pretty_env_logger;

pub mod context;
pub mod modules;
pub mod print;
pub mod segment;
