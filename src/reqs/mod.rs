use std::collections::{BTreeMap as BMap, HashMap};
use reqwest::{Client, get};
use serde::{Serialize, Deserialize};
use serde_json::{Value, to_string};
use crate::files_utils;

#[derive(Deserialize, Serialize)]
pub struct ResponseApi {
    pub tokens: Vec<String>,
    pub total: i64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseVivo {
    msisdn: Value,
    campaign_code: Value,
    op_code: Value,
    #[serde(flatten)]
    others: BMap<String, Value>,
    #[serde(alias = "return")]
    status: bool
}

const URL_BASE: &'static str = "URL-DE-SUA-API";

pub async fn get_tokens() -> Result<String, Box<dyn std::error::Error>> {
    let req = get(format!("{}/tokens", URL_BASE))
        .await?;

    let res: ResponseApi = req.json()
        .await?;

    to_string(&res)
        .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
}

pub async fn register_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = get_tokens()
        .await?;
    files_utils::write_file(".data.json", tokens);
    Ok(())
}


pub async fn request_to_api(number: String, token: String) -> Result<bool, Box<dyn std::error::Error>> {

    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("msisdn", number);
    params.insert("campid", token);
    params.insert("opCode", "VV".to_string());

    let res = client.post("http://interatividade.vivo.ddivulga.com/carrotProcess")
        .form(&params)
        .send()
        .await?;

    let res_json: ResponseVivo = res.json()
        .await?;

    Ok(res_json.status)
}
