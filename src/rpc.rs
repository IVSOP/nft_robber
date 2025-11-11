use anyhow::{Result, anyhow};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct GetAccountInfoResponse {
    pub lamports: u64,
    /// Usually [data, encoding(like base64)]
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub space: u64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetAccountInfo {
    pub data: Option<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u64,
}

pub struct Rpc {
    pub client: Client,
    pub url: String,
}

impl Rpc {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn get_account_info(&self, pubkey: &str) -> Result<Option<GetAccountInfoResponse>> {
        let json_body = format!(
            r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getAccountInfo",
            "params": ["{}"]
        }}"#,
            pubkey
        );

        info!("Getting info from account {}", pubkey);

        let res = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await?;

        let text = res.text().await?;

        // Parse the response as a generic JSON value
        let v: Value = serde_json::from_str(&text)?;

        // If it has an error, deserialize it and return it
        if let Some(err) = v.get("error") {
            let rpc_error: RpcError = serde_json::from_value(err.clone())?;
            return Err(anyhow!(
                "RPC Error {}: {}",
                rpc_error.code,
                rpc_error.message
            ));
        }

        if let Some(result) = v.get("result") {
            // if it has a value, but the value is null, return None
            if result.get("value").is_none() || result.get("value").unwrap().is_null() {
                return Ok(None);
            }

            // if the value has info, return it
            let account_info: GetAccountInfoResponse =
                serde_json::from_value(result.get("value").unwrap().clone())?;
            return Ok(Some(account_info));
        }

        // "result" not found in the response, but there was also no error
        Err(anyhow!("Unexpected RPC response: {}", text))
    }

    pub async fn set_account_info(&self, pubkey: &str, info: &SetAccountInfo) -> Result<()> {
        let json_body = format!(
            r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "surfnet_setAccount",
            "params": ["{pubkey}", {}]
        }}"#,
            serde_json::to_string(info)?
        );

        info!("Setting info for account {}", pubkey);

        let res = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await?;

        let text = res.text().await?;

        // Parse the response as a generic JSON value
        let v: Value = serde_json::from_str(&text)?;

        // If it has an error, deserialize it and return it
        if let Some(err) = v.get("error") {
            let rpc_error: RpcError = serde_json::from_value(err.clone())?;
            return Err(anyhow!(
                "RPC Error {}: {}",
                rpc_error.code,
                rpc_error.message
            ));
        }

        Ok(())
    }

    pub async fn close_account(&self, pubkey: &str) -> Result<()> {
        self.set_account_info(
            pubkey,
            &SetAccountInfo {
                data: Some(String::new()),
                executable: false,
                lamports: 0,
                owner: "11111111111111111111111111111111".into(),
                rent_epoch: 0,
            },
        )
        .await
    }
}
