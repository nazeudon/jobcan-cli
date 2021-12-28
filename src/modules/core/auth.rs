use crate::modules::credential::Credential;
use crate::modules::utils::{confirm, input_email, input_password};
use std::error::Error;

pub fn auth() -> Result<(), Box<dyn Error>> {
    let email = input_email()?;
    let password = input_password()?;
    confirm()?;

    let credential = Credential::new(email, password);
    credential.to_json()?;

    Ok(())
}
