use std::{error::Error, collections::HashMap, time::{Duration, Instant}, str::FromStr};

use reqwest::{RequestBuilder, Method, Url, header::{HeaderMap, HeaderValue, HeaderName}};
use serde_json::Value;

use crate::cli_lib::{request::ParsedRequest, parse_file};

pub async fn handle_send(files_path: &Vec<String>, _write: bool) {
  let start = Instant::now();
    
  let requests: Vec<ParsedRequest> = 
    files_path
      .iter()
      .map(|file| match parse_file(file) {
        Ok(data) => data,
        Err(error) => 
          panic!("FAILED AT FILES PARSING:\nERROR::{:?}", error)
      }).collect()
  ;

  for request in requests {
    let response = match send_request(&request).await {
      Ok(data) => data,
      Err(error) => 
        panic!("FAILED AT REQUEST SENDING:\nERROR::{:?}", error),
    };

    println!("{}", response);
  }

  println!("TOOK: {:?}", start.elapsed());
}

async fn send_request(request: &ParsedRequest) -> Result<String, Box<dyn Error>> {
  let response = build_request(request)
    ?.send()
    .await?
    .json::<HashMap<String, Value>>()
    .await?
  ;

  let serialized_resp = serde_json::to_string(&response)?;

  return Ok(serialized_resp);
}

fn build_request(request: &ParsedRequest) -> Result<RequestBuilder, Box<dyn Error>> {
  let client = reqwest::Client::new();

  let method = match request.method.to_uppercase().as_str() {
    "GET" => Method::GET,
    "POST" => Method::POST,
    "PUT" => Method::PUT,
    "PATCH" => Method::PATCH,
    "DELETE" => Method::DELETE,
    "HEADER" => Method::HEAD,
    "OPTIONS" => Method::OPTIONS,
    _ => panic!("Invalid Method: `{}`", request.method),
  };

  let url = Url::parse(&request.url)?;

  let mut req = client.request(method, url);

  if let Some(timeout) = request.timeout {
    req = req.timeout(Duration::from_millis(timeout.into()));
  }

  if let Some(body) = &request.body {
    let string_body = serde_json::to_string(body)?;
    req = req.body(string_body);
  }

  if let Some(headers) = &request.headers {
    let header_map: Result<HeaderMap, Box<dyn Error>> = 
      headers
        .iter()
        .map(|(key, val)| {
          let header_val = HeaderValue::from_str(&val)?;
          let header_key = HeaderName::from_str(&key)?;

          Ok((header_key, header_val))
        })
        .collect()
    ;

    req = req.headers(header_map?);
  }

  return Ok(req);
}