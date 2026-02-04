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
