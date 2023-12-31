// LINK - https://docs.rs/http/latest/http/request/struct.Request.html#:~:text=use%20http%3A%3A%7BRequest%2C%20Response,let%20response%20%3D%20send(request.

use std::{fs, error::Error};

use crate::lib::request::Request;

pub fn handle_send(files_path: &Vec<String>, write: bool) {
  println!("{:?}, {}", files_path, write);

  let data: Vec<Request> = 
    files_path
    .iter()
    .map(|file| match parse_file(file) {
      Ok(data) => data,
      Err(error) => panic!("FAILED AT FILES PARSING:\nERROR::{:?}", error)
    }).collect()
  ;

  println!("{:?}", data)
}

fn parse_file(file_path: &String) -> Result<Request, Box<dyn Error>> {
  let file_string = fs::read_to_string(file_path)?;
  let data: Request = serde_json::from_str(&file_string)?;

  return Ok(data)
}
