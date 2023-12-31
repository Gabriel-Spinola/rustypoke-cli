pub fn handle_build(file_path: &Vec<String>, output_path: &Option<String>) {
  println!("{:?}, {}", file_path, output_path.as_deref().unwrap_or("Output's not defined"))
}