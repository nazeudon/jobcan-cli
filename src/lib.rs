extern crate image;
pub mod modules;

use modules::config::{Config, Mode};
use modules::core::exec::{end, start};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.mode {
        Mode::Start => start(),
        Mode::End => end(),
    }
}
