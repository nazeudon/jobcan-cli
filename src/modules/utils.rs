use headless_chrome::protocol::page::ScreenshotFormat;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

pub fn save_screen(tab: Arc<headless_chrome::Tab>) -> Result<(), Box<dyn Error>> {
    let mut _png_data = tab
        .wait_for_element("body")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    let png_data = &_png_data[..];
    let mut file = File::create("screenshot.png")?;
    file.write_all(png_data)?;

    Ok(())
}
