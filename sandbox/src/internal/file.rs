/// Checks if file exists and returns true or false
pub fn file_exists(filename: &str) -> bool {
  match std::fs::read(filename) {
    Ok(_) => true,
    Err(_) => false
  }
}

/// Creates file and returns true or false if it's successful or not
pub fn create_file(filename: &str) -> bool {
  match std::fs::File::create(filename) {
    Ok(_) => true,
    Err(_) => false
  }
}