//! Cellphone Login Module
//!
//! # Usage
//!
//! This module provides functionality to log in using a mobile phone number and SMS captcha.
//!
//! ## Workflow
//!
//! 1. **Send Captcha**: Call the [`send_captcha`] function to send an SMS verification code to the specified phone number.
//! 2. **Login**: Once the user receives the code, call the [`login_cellphone`] function with the phone number and the captcha to complete the login.
//!    - [`login_cellphone`] handles the encryption and request processing internally.
//!    - Although [`verify_captcha`] is available for standalone verification, it is not required to call it before logging in.
//!
//! ## Handling Cookies
//!
//! The [`LoginResult`] struct returned by [`login_cellphone`] contains a `cookies` field.
//! These cookies (e.g., `MUSIC_U`) are essential credentials for making subsequent authenticated API calls.
//! It is recommended to save these cookies to the `reqwest::Client`'s Cookie Store or manually attach them to headers in future requests.
//!
//! # Example
//!
//! ```rust,no_run
//! use reqwest::Client;
//! // Assuming the crate name is `music_w`
//! use music_w::enhanced::login_cellphone::{send_captcha, login_cellphone};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::new();
//!     let phone = "13800138000";
//!     let country_code = Some("86");
//!
//!     // 1. Send Captcha
//!     send_captcha(&client, phone, country_code).await?;
//!     
//!     // ... Retrieve the captcha entered by the user ...
//!     let captcha = "1234";
//!
//!     // 2. Login
//!     let result = login_cellphone(&client, phone, captcha, country_code).await?;
//!     
//!     println!("Login successful, status code: {}", result.status);
//!     println!("Received Cookies: {:?}", result.cookies);
//!     
//!     Ok(())
//! }
//! ```

mod inner;

use anyhow::Result;
use reqwest::Client;

pub use inner::{LoginCellphoneResponse, LoginResult};

/// Send SMS captcha to the specified phone number.
///
/// Returns `Ok(true)` if the captcha was sent successfully (API code 200).
pub async fn send_captcha(
    client: &Client,
    phone: &str,
    ctcode: Option<&str>,
) -> Result<bool> {
    let resp =
        inner::send_captcha_inner(client, phone, ctcode)
            .await?;
    Ok(resp.code == 200)
}

/// Verify SMS captcha for the specified phone number.
///
/// Returns `Ok(true)` if the captcha was verified successfully (API code 200).
pub async fn verify_captcha(
    client: &Client,
    phone: &str,
    captcha: &str,
    ctcode: Option<&str>,
) -> Result<bool> {
    let resp = inner::verify_captcha_inner(
        client, phone, captcha, ctcode,
    )
    .await?;
    Ok(resp.code == 200)
}

/// Perform cellphone login using the provided credentials.
pub async fn login_cellphone(
    client: &Client,
    phone: &str,
    captcha: &str,
    ctcode: Option<&str>,
) -> Result<LoginResult> {
    inner::login_cellphone_inner(
        client, phone, captcha, ctcode,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    /// Integration test for sending SMS captcha
    ///
    /// Requires `CELLPHONE_NUMBER` environment variable.
    #[tokio::test]
    #[ignore]
    async fn test_integration_send_captcha() {
        dotenv().ok();
        let phone_number = env::var("CELLPHONE_NUMBER").expect("CELLPHONE_NUMBER environment variable not set for integration test");
        let client = Client::new();
        let result = send_captcha(
            &client,
            &phone_number,
            Some("86"),
        )
        .await;

        match result {
            Ok(success) => {
                if !success {
                    println!(
                        "Warning: captcha sent but success was false (maybe code != 200)"
                    );
                }
            }
            Err(e) => {
                panic!("Integration test failed: {:?}", e)
            }
        }
    }
}
