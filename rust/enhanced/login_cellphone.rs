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

    /// Interactive integration test for full login flow
    ///
    /// This test will:
    /// 1. Send a captcha to the phone number specified in `CELLPHONE_NUMBER`.
    /// 2. Wait for user input from stdin.
    /// 3. Attempt to log in with the provided captcha.
    ///
    /// cargo test --package music_w --lib -- enhanced::login_cellphone::tests::test_interactive_login_flow --ignored --nocapture
    #[tokio::test]
    #[ignore]
    async fn test_interactive_login_flow() {
        use std::io::{self, Write};
        
        dotenv().ok();
        let phone_number = env::var("CELLPHONE_NUMBER").expect("CELLPHONE_NUMBER environment variable not set");
        let client = Client::new();

        println!("\n--- Interactive Login Test ---");
        println!("Target Phone: {}", phone_number);

        // 1. Send Captcha
        println!("Sending captcha...");
        let send_res = send_captcha(&client, &phone_number, Some("86")).await;
        match send_res {
            Ok(true) => println!("Captcha sent successfully!"),
            Ok(false) => {
                eprintln!("Failed to send captcha (API returned non-200).");
                return;
            }
            Err(e) => {
                eprintln!("Error sending captcha: {:?}", e);
                return;
            }
        }

        // 2. Wait for input
        print!("Please enter the SMS captcha you received: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let captcha = input.trim();

        if captcha.is_empty() {
            println!("No captcha entered, aborting.");
            return;
        }

        // 3. Login
        println!("Logging in with captcha '{}'...", captcha);
        let login_res = login_cellphone(&client, &phone_number, captcha, Some("86")).await;

        match login_res {
            Ok(result) => {
                println!("Login SUCCESS!");
                println!("Status: {}", result.status);
                println!("Cookies: {:?}", result.cookies);
                // Basic assertion
                assert_eq!(result.status, 200, "Login status should be 200");
            }
            Err(e) => {
                println!("Login FAILED: {:?}", e);
                panic!("Login failed");
            }
        }
    }
}
