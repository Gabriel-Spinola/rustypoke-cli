use std::{collections::HashMap, fmt::Debug};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Request {
  url: String,
  query_params: Option<HashMap<String, String>>,
  method: String,
  headers: Option<HashMap<String, String>>,
  body: Option<HashMap<String, serde_json::Value>>,
  // TODO - cookies: HashMap<String, http::Cookie>,
  timeout: Option<u32>,
  allow_redirects: Option<bool>,
  proxies: Option<HashMap<String, String>>,
  verify_tls: Option<bool>,
}
