use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request payload for sending a verification code.
#[derive(Serialize, Debug)]
pub struct SendCaptchaRequest {
    #[serde(rename = "ctcode")]
    pub country_code: String,
    pub secrete: String,
    pub cellphone: String,
}

/// Response structure for sending a verification code.
#[derive(Deserialize, Debug)]
pub struct SendCaptchaResponse {
    pub code: i32,
    pub message: Option<String>,
}

/// Request payload for verifying a verification code.
#[derive(Serialize, Debug)]
pub struct VerifyCaptchaRequest {
    #[serde(rename = "ctcode")]
    pub country_code: String,
    pub cellphone: String,
    pub captcha: String,
}

/// Response structure for verifying a verification code.
#[derive(Deserialize, Debug)]
pub struct VerifyCaptchaResponse {
    pub code: i32,
    pub message: Option<String>,
}

/// Request payload for logging in with a phone number and captcha.
#[derive(Serialize, Debug)]
pub struct LoginCellphoneRequest {
    #[serde(rename = "type")]
    pub login_type: String, // '1' for cellphone login
    pub https: String, // 'true'
    pub phone: String,
    #[serde(rename = "countrycode")]
    pub country_code: String,
    pub captcha: String,
    pub remember: String, // 'true'
}

/// Response structure for successful cellphone login.
#[derive(Deserialize, Debug)]
pub struct LoginCellphoneResponse {
    #[serde(rename = "avatarImgIdStr")]
    pub avatar_img_id_str: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents the overall login result, including cookies.
#[derive(Debug)]
pub struct LoginResult {
    pub status: i32,
    pub body: LoginCellphoneResponse,
    pub cookies: Vec<String>,
}

const BASE_URL: &str = "https://music.163.com";

/// Placeholder for the weapi encryption logic.
/// In a real implementation, this would encrypt the data and return the encrypted payload.
fn encrypt_weapi(
    data: &impl Serialize,
) -> Result<HashMap<String, String>> {
    // This is a placeholder. NetEase weapi requires complex AES and RSA encryption.
    // For now, we return the data as-is in a way that reqwest can send,
    // but in reality, this would be { "params": "...", "encSecKey": "..." }
    let mut map = HashMap::new();
    let json = serde_json::to_string(data)?;
    map.insert("params".to_string(), json); // Fake encryption
    map.insert(
        "encSecKey".to_string(),
        "placeholder".to_string(),
    );
    Ok(map)
}

/// Sends a verification code to the specified phone number.
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

    let encrypted_data = encrypt_weapi(&data)?;

    let resp = client
        .post(&url)
        .form(&encrypted_data)
        .send()
        .await?
        .json::<SendCaptchaResponse>()
        .await?;

    Ok(resp)
}

/// Verifies the verification code for the specified phone number.
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

    let encrypted_data = encrypt_weapi(&data)?;

    let resp = client
        .post(&url)
        .form(&encrypted_data)
        .send()
        .await?
        .json::<VerifyCaptchaResponse>()
        .await?;

    Ok(resp)
}

/// Logs in using a phone number and verification code.
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

    let encrypted_data = encrypt_weapi(&data)?;

    let response = client
        .post(&url)
        .form(&encrypted_data)
        .send()
        .await?;

    let status = response.status().as_u16() as i32;

    // Extract cookies
    let cookies = response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .collect();

    let body =
        response.json::<LoginCellphoneResponse>().await?;

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

    #[test]
    fn test_encrypt_weapi_placeholder() {
        let data_map = HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);
        let encrypted_result =
            encrypt_weapi(&data_map).unwrap();

        let expected_params =
            serde_json::to_string(&data_map).unwrap();
        let expected_enc_sec_key =
            "placeholder".to_string();

        assert_eq!(
            encrypted_result.get("params"),
            Some(&expected_params)
        );
        assert_eq!(
            encrypted_result.get("encSecKey"),
            Some(&expected_enc_sec_key)
        );
    }
}
