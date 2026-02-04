use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::enhanced::crypto::weapi;

/// Request structure for sending SMS captcha to cellphone
#[derive(Serialize, Debug)]
pub struct SendCaptchaRequest {
    /// Country code for the phone number (renamed to "ctcode" for API compatibility)
    #[serde(rename = "ctcode")]
    pub country_code: String,
    /// Secret key for the captcha service
    pub secrete: String,
    /// Phone number to send captcha to
    pub cellphone: String,
}

/// Response structure for sending SMS captcha
#[derive(Deserialize, Debug)]
pub struct SendCaptchaResponse {
    /// API response code (0 for success, non-zero for errors)
    pub code: i32,
    /// Optional error message or success message
    pub message: Option<String>,
}

/// Request structure for verifying SMS captcha
#[derive(Serialize, Debug)]
pub struct VerifyCaptchaRequest {
    /// Country code for the phone number (renamed to "ctcode" for API compatibility)
    #[serde(rename = "ctcode")]
    pub country_code: String,
    /// Phone number to verify captcha for
    pub cellphone: String,
    /// Captcha code received on the phone
    pub captcha: String,
}

/// Response structure for verifying SMS captcha
#[derive(Deserialize, Debug)]
pub struct VerifyCaptchaResponse {
    /// API response code (0 for success, non-zero for errors)
    pub code: i32,
    /// Optional error message or success message
    pub message: Option<String>,
}

/// Request structure for cellphone login
#[derive(Serialize, Debug)]
pub struct LoginCellphoneRequest {
    /// Login type (renamed to "type" for API compatibility)
    #[serde(rename = "type")]
    pub login_type: String,
    /// HTTPS flag for the request
    pub https: String,
    /// Phone number for login
    pub phone: String,
    /// Country code for the phone number (renamed to "countrycode" for API compatibility)
    #[serde(rename = "countrycode")]
    pub country_code: String,
    /// Captcha code for verification
    pub captcha: String,
    /// Remember login flag
    pub remember: String,
}

/// Response structure for cellphone login
#[derive(Deserialize, Debug)]
pub struct LoginCellphoneResponse {
    /// Avatar image ID string (renamed to "avatarImgIdStr" for API compatibility)
    #[serde(rename = "avatarImgIdStr")]
    pub avatar_img_id_str: Option<String>,
    /// Additional response data from the API
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Result structure for cellphone login operation
#[derive(Debug)]
pub struct LoginResult {
    /// HTTP status code of the login request
    pub status: i32,
    /// Response body from the login API
    pub body: LoginCellphoneResponse,
    /// Cookies received from the login response
    pub cookies: Vec<String>,
}

const BASE_URL: &str = "https://music.163.com";

/// Send SMS captcha to the specified phone number
///
/// This function sends a captcha to the provided phone number for verification.
/// It uses the NetEase Music API to send the SMS captcha.
///
/// # Parameters
/// - `client`: The HTTP client to use for the request
/// - `phone`: The phone number to send the captcha to
/// - `ctcode`: Optional country code for the phone number (defaults to "86" if not provided)
///
/// # Returns
/// Returns a `SendCaptchaResponse` containing the API response code and message.
///
/// # Errors
/// Returns an error if the HTTP request fails or the response cannot be parsed.
pub async fn send_captcha(
    client: &Client,
    phone: &str,
    ctcode: Option<&str>,
) -> Result<SendCaptchaResponse> {
    let url =
        format!("{}/weapi/sms/captcha/sent", BASE_URL);
    let data = SendCaptchaRequest {
        country_code: ctcode.unwrap_or("86").to_string(),
        secrete: "music_middleuser_pclogin".to_string(),
        cellphone: phone.to_string(),
    };

    // Encrypt the request data using WeAPI encryption for NetEase Music API
    let encrypted_data = weapi(&data)?;

    let response = client
        .post(&url)
        .form(&encrypted_data)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .header(reqwest::header::REFERER, "https://music.163.com")
        .header(reqwest::header::ORIGIN, "https://music.163.com")
        .send()
        .await?;

    let text = response.text().await?;
    let result: SendCaptchaResponse = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("Failed to parse SendCaptchaResponse: {}, body: {}", e, text))?;
    Ok(result)
}

/// Verify SMS captcha for the specified phone number
///
/// This function verifies the captcha code received on the phone number.
/// It uses the NetEase Music API to verify the SMS captcha.
///
/// # Parameters
/// - `client`: The HTTP client to use for the request
/// - `phone`: The phone number to verify the captcha for
/// - `captcha`: The captcha code received on the phone
/// - `ctcode`: Optional country code for the phone number (defaults to "86" if not provided)
///
/// # Returns
/// Returns a `VerifyCaptchaResponse` containing the API response code and message.
///
/// # Errors
/// Returns an error if the HTTP request fails or the response cannot be parsed.
pub async fn verify_captcha(
    client: &Client,
    phone: &str,
    captcha: &str,
    ctcode: Option<&str>,
) -> Result<VerifyCaptchaResponse> {
    let url =
        format!("{}/weapi/sms/captcha/verify", BASE_URL);
    let data = VerifyCaptchaRequest {
        country_code: ctcode.unwrap_or("86").to_string(),
        cellphone: phone.to_string(),
        captcha: captcha.to_string(),
    };

    let encrypted_data = weapi(&data)?;

    let response = client
        .post(&url)
        .form(&encrypted_data)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .header(reqwest::header::REFERER, "https://music.163.com")
        .header(reqwest::header::ORIGIN, "https://music.163.com")
        .send()
        .await?;

    let text = response.text().await?;
    let result: VerifyCaptchaResponse = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("Failed to parse VerifyCaptchaResponse: {}, body: {}", e, text))?;
    Ok(result)
}

/// Perform cellphone login using the provided credentials
///
/// This function performs a cellphone login to NetEase Music using the provided phone number and captcha.
/// It uses the NetEase Music API to authenticate the user.
///
/// # Parameters
/// - `client`: The HTTP client to use for the request
/// - `phone`: The phone number for login
/// - `captcha`: The captcha code for verification
/// - `ctcode`: Optional country code for the phone number (defaults to "86" if not provided)
///
/// # Returns
/// Returns a `LoginResult` containing the HTTP status, response body, and cookies.
///
/// # Errors
/// Returns an error if the HTTP request fails, the response cannot be parsed, or login fails.
pub async fn login_cellphone(
    client: &Client,
    phone: &str,
    captcha: &str,
    ctcode: Option<&str>,
) -> Result<LoginResult> {
    let url =
        format!("{}/weapi/w/login/cellphone", BASE_URL);
    let data = LoginCellphoneRequest {
        login_type: "1".to_string(),
        https: "true".to_string(),
        phone: phone.to_string(),
        country_code: ctcode.unwrap_or("86").to_string(),
        captcha: captcha.to_string(),
        remember: "true".to_string(),
    };

    let encrypted_data = weapi(&data)?;

    let response = client
        .post(&url)
        .form(&encrypted_data)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .header(reqwest::header::REFERER, "https://music.163.com")
        .header(reqwest::header::ORIGIN, "https://music.163.com")
        .send()
        .await?;

    let status = response.status().as_u16() as i32;

    // Extract cookies from the response headers
    let cookies = response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .collect();

    let text = response.text().await?;
    // Parse the JSON response body, with detailed error reporting
    let body: LoginCellphoneResponse = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("Failed to parse LoginCellphoneResponse: {}, body: {}", e, text))?;

    Ok(LoginResult {
        status,
        body,
        cookies,
    })
}

/// Tests module for cellphone login functionality
///
/// This module contains unit tests for the cellphone login functionality,
/// including request serialization tests and integration tests.
#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use serde_json::json;
    use std::env;

    /// Test SendCaptchaRequest serialization
    ///
    /// This test verifies that the SendCaptchaRequest struct serializes correctly
    /// to the expected JSON format with proper field renaming.
    #[test]
    fn test_send_captcha_request_serialization() {
        let request = SendCaptchaRequest {
            country_code: "1".to_string(),
            secrete: "test_secrete".to_string(),
            cellphone: "1234567890".to_string(),
        };
        let serialized =
            serde_json::to_value(&request).unwrap();
        let expected = json!({
            "ctcode": "1",
            "secrete": "test_secrete",
            "cellphone": "1234567890"
        });
        assert_eq!(serialized, expected);
    }

    /// Test VerifyCaptchaRequest serialization
    ///
    /// This test verifies that the VerifyCaptchaRequest struct serializes correctly
    /// to the expected JSON format with proper field renaming.
    #[test]
    fn test_verify_captcha_request_serialization() {
        let request = VerifyCaptchaRequest {
            country_code: "1".to_string(),
            cellphone: "1234567890".to_string(),
            captcha: "123456".to_string(),
        };
        let serialized =
            serde_json::to_value(&request).unwrap();
        let expected = json!({
            "ctcode": "1",
            "cellphone": "1234567890",
            "captcha": "123456"
        });
        assert_eq!(serialized, expected);
    }

    /// Test LoginCellphoneRequest serialization
    ///
    /// This test verifies that the LoginCellphoneRequest struct serializes correctly
    /// to the expected JSON format with proper field renaming.
    #[test]
    fn test_login_cellphone_request_serialization() {
        let request = LoginCellphoneRequest {
            login_type: "1".to_string(),
            https: "true".to_string(),
            phone: "1234567890".to_string(),
            country_code: "1".to_string(),
            captcha: "123456".to_string(),
            remember: "true".to_string(),
        };
        let serialized =
            serde_json::to_value(&request).unwrap();
        let expected = json!({
            "type": "1",
            "https": "true",
            "phone": "1234567890",
            "countrycode": "1",
            "captcha": "123456",
            "remember": "true"
        });
        assert_eq!(serialized, expected);
    }

    /// Integration test for sending SMS captcha
    ///
    /// This test sends a real SMS captcha to a phone number using the NetEase Music API.
    /// It requires the CELLPHONE_NUMBER environment variable to be set.
    /// The test is ignored by default and should be run manually when needed.
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

        result.unwrap_or_else(|e| {
            panic!("Integration test failed: {:?}", e)
        });
    }
}
