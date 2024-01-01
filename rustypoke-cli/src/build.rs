use std::time::Instant;

use serde_json::Value;

use crate::cli_lib::parse_file;

pub fn handle_build(files_path: &Vec<String>, output_path: &Option<String>) {
  let start = Instant::now();
    
  let requests: Value = 
    files_path
      .iter()
      .map(|file| match parse_file(file) {
        Ok(data) => data,
        Err(error) => 
          panic!("FAILED AT FILES PARSING:\nERROR::{:?}", error)
      }).collect()
  ;

  println!("TOOK: {:?}", start.elapsed());
}