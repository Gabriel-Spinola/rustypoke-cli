use self::request::ParsedRequest;
use std::{fs, error::Error};

pub mod request;

pub fn parse_file(file_path: &String) -> Result<ParsedRequest, Box<dyn Error>> {
  let file_string = fs::read_to_string(file_path)?;
  let data: ParsedRequest = serde_json::from_str(&file_string)?;

  return Ok(data)
}