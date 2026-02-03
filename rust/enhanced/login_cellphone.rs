use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::enhanced::crypto::weapi;

#[derive(Serialize, Debug)]
pub struct SendCaptchaRequest {
    #[serde(rename = "ctcode")]
    pub country_code: String,
    pub secrete: String,
    pub cellphone: String,
}

#[derive(Deserialize, Debug)]
pub struct SendCaptchaResponse {
    pub code: i32,
    pub message: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct VerifyCaptchaRequest {
    #[serde(rename = "ctcode")]
    pub country_code: String,
    pub cellphone: String,
    pub captcha: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyCaptchaResponse {
    pub code: i32,
    pub message: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginCellphoneRequest {
    #[serde(rename = "type")]
    pub login_type: String,
    pub https: String,
    pub phone: String,
    #[serde(rename = "countrycode")]
    pub country_code: String,
    pub captcha: String,
    pub remember: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginCellphoneResponse {
    #[serde(rename = "avatarImgIdStr")]
    pub avatar_img_id_str: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
pub struct LoginResult {
    pub status: i32,
    pub body: LoginCellphoneResponse,
    pub cookies: Vec<String>,
}

const BASE_URL: &str = "https://music.163.com";

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
    use dotenvy::dotenv;
    use serde_json::json;
    use std::env;

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
