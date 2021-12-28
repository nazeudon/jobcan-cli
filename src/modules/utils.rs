use dialoguer::{Confirm, Input, Password};
use headless_chrome::protocol::page::ScreenshotFormat;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

pub fn input_email() -> Result<String, Box<dyn Error>> {
    let email = Input::new()
        .with_prompt("メールアドレスを入力")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') {
                Ok(())
            } else {
                Err("メールアドレスを正しく入力してください")
            }
        })
        .interact()?;

    Ok(email)
}

pub fn input_password() -> Result<String, Box<dyn Error>> {
    let password = Password::new()
        .with_prompt("パスワードを入力")
        .with_confirmation("パスワードを再入力(確認用)", "パスワードが一致しません")
        .interact()?;

    Ok(password)
}

pub fn confirm() -> Result<(), Box<dyn Error>> {
    if Confirm::new()
        .with_prompt("上記の内容で保存しますか?")
        .interact()?
    {
        println!("認証情報を保存しました!");
    } else {
        println!("キャンセルしました");
    }

    Ok(())
}

pub fn save_screen(tab: Arc<headless_chrome::Tab>) -> Result<(), Box<dyn Error>> {
    let mut _png_data = tab
        .wait_for_element("body")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    let png_data = &_png_data[..];
    let mut file = File::create("screenshot.png")?;
    file.write_all(png_data)?;

    Ok(())
}
