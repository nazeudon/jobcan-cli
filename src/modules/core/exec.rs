use crate::modules::core::login::login;
use crate::modules::error::PageTransitionError;
use crate::modules::utils::save_screen;
use std::error::Error;

const LOGIN_URL: &str = "https://ssl.jobcan.jp/jbcoauth/login";
const EMPLOYEE_URL: &str = "https://ssl.jobcan.jp/employee";

pub fn start() -> Result<(), Box<dyn Error>> {
    let result = login();

    match result {
        Ok(browser) => {
            let new_tab = browser.new_tab()?;
            new_tab.navigate_to(LOGIN_URL)?;
            new_tab.wait_until_navigated()?;
            match new_tab.get_url().as_str() {
                EMPLOYEE_URL => {
                    new_tab
                        .wait_for_element("#adit-button-work-start")?
                        .click()?;
                    save_screen(new_tab)?;
                    println!("Success: 出勤しました!");
                }
                _ => return Err(Box::new(PageTransitionError())),
            }
        }
        Err(err) => return Err(err),
    };

    Ok(())
}

pub fn end() -> Result<(), Box<dyn Error>> {
    let result = login();

    match result {
        Ok(browser) => {
            let new_tab = browser.new_tab()?;
            new_tab.navigate_to(LOGIN_URL)?;
            new_tab.wait_until_navigated()?;
            match new_tab.get_url().as_str() {
                EMPLOYEE_URL => {
                    new_tab.wait_for_element("#adit-button-work-end")?.click()?;
                    save_screen(new_tab)?;
                    println!("Successed: 退勤しました!");
                }
                _ => return Err(Box::new(PageTransitionError())),
            }
        }
        Err(err) => return Err(err),
    };

    Ok(())
}
