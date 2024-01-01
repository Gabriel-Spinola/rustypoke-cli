use std::{fs, error::Error};
use serde::Deserialize;

pub mod request;

pub fn parse_file<T>(file_path: &str) -> Result<T, Box<dyn Error>> 
where 
  T: for<'a> Deserialize<'a>
{
  let file_string = fs::read_to_string(file_path)?;
  let data: T = serde_json::from_str(&file_string)?;

  return Ok(data);
}