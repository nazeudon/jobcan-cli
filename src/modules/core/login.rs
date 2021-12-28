use crate::modules::credential::Credential;
use headless_chrome::Browser;
use std::error::Error;

const SIGNIN_URL: &str = "https://id.jobcan.jp/users/sign_in";

pub fn login() -> Result<Browser, Box<dyn Error>> {
    let browser = Browser::default()?;
    let credential = Credential::from_json()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(SIGNIN_URL)?;
    tab.wait_for_element("input#user_email")?.click()?;
    tab.type_str(&credential.email)?;
    tab.wait_for_element("input#user_password")?.click()?;
    tab.type_str(&credential.password)?;
    tab.wait_for_element("input.form__login")?.click()?;

    Ok(browser)
}
