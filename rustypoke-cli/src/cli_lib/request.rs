use std::{collections::HashMap, fmt::Debug};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ParsedRequest {
  pub url: String,
  pub query_params: Option<HashMap<String, String>>,
  pub method: String,
  pub headers: Option<HashMap<String, String>>,
  pub body: Option<HashMap<String, serde_json::Value>>,
  // TODO - cookies: HashMap<String, http::Cookie>,
  pub timeout: Option<u32>,
  // TODO - pub allow_redirects: Option<bool>,
  // TODO - pub proxies: Option<HashMap<String, String>>,
  // TODO - pub verify_tls: Option<bool>,
}
