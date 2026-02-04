use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::crypto::weapi;

/// Request structure for sending SMS captcha to cellphone
#[derive(Serialize, Debug)]
pub(crate) struct SendCaptchaRequest {
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
pub(crate) struct SendCaptchaResponse {
    /// API response code (0 for success, non-zero for errors)
    pub code: i32,
    /// Optional error message or success message
    pub message: Option<String>,
}

/// Request structure for verifying SMS captcha
#[derive(Serialize, Debug)]
pub(crate) struct VerifyCaptchaRequest {
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
pub(crate) struct VerifyCaptchaResponse {
    /// API response code (0 for success, non-zero for errors)
    pub code: i32,
    /// Optional error message or success message
    pub message: Option<String>,
}

/// Request structure for cellphone login
#[derive(Serialize, Debug)]
pub(crate) struct LoginCellphoneRequest {
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
#[derive(Deserialize, Debug, Clone)]
pub struct LoginCellphoneResponse {
    /// Avatar image ID string (renamed to "avatarImgIdStr" for API compatibility)
    #[serde(rename = "avatarImgIdStr")]
    pub avatar_img_id_str: Option<String>,
    /// Additional response data from the API
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Result structure for cellphone login operation
#[derive(Debug, Clone)]
pub struct LoginResult {
    /// HTTP status code of the login request
    pub status: i32,
    /// Response body from the login API
    pub body: LoginCellphoneResponse,
    /// Cookies received from the login response
    pub cookies: Vec<String>,
}

pub(crate) const BASE_URL: &str = "https://music.163.com";

/// Send SMS captcha to the specified phone number
pub(crate) async fn send_captcha_inner(
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
pub(crate) async fn verify_captcha_inner(
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
pub(crate) async fn login_cellphone_inner(
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

    let cookies = response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .collect();

    let text = response.text().await?;
    let body: LoginCellphoneResponse = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("Failed to parse LoginCellphoneResponse: {}, body: {}", e, text))?;

    Ok(LoginResult {
        status,
        body,
        cookies,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
}
