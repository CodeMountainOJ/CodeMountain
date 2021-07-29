use image::load_from_memory;

pub fn validate_img(bytes: &Vec<u8>) -> bool {
    match load_from_memory(bytes) {
        Ok(_) => true,
        Err(_) => false
    }
}