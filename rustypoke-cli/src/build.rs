use std::{time::Instant, fs::File, error::Error, io::{BufWriter, Write}};

use crate::cli_lib::{parse_file, request::{ParsedRequest, Body}};

pub fn handle_build(files_path: &Vec<String>, output_path: &str) {
  let start = Instant::now();
    
  let body: Vec<Body> = 
    files_path
      .iter()
      .map(|file| match parse_file(file) {
        Ok(data) => data,
        Err(error) => 
          panic!("FAILED AT FILES PARSING:\nERROR::{:?}", error)
      }).collect()
  ;

  let request_body = ParsedRequest { 
    url: "".to_string(),
    query_params: None,
    method: "".to_string(),
    headers: None, 
    body: Some(body[0].clone()), 
    timeout: None, 
  };

  match create_output(&request_body, output_path) {
    Ok(data) => println!("{:?}", data),
    Err(error) => println!("Failed to create output\nERROR::{}", error),
  };

  println!("TOOK: {:?}", start.elapsed());
}

fn create_output(data: &ParsedRequest, output_path: &str) -> Result<(), Box<dyn Error>>{
  let file = File::create(output_path)?;
  let mut writer = BufWriter::new(file);
  
  serde_json::to_writer(&mut writer, &data)?;
  writer.flush()?;

  return Ok(());
}