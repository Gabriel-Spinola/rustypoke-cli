use std::{collections::HashMap, fmt::Debug};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ParsedRequest {
  pub url: String,
  pub query_params: Option<HashMap<String, String>>,
  pub method: String,
  pub headers: Option<HashMap<String, String>>,
  pub body: Option<Body>,
  // TODO - cookies: HashMap<String, http::Cookie>,
  pub timeout: Option<u32>,
  // TODO - pub allow_redirects: Option<bool>,
  // TODO - pub proxies: Option<HashMap<String, String>>,
  // TODO - pub verify_tls: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Body(HashMap<String, Value>);