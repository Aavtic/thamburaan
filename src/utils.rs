use std::fs;


pub fn dir_exists(dir_name: &str) -> bool{
    fs::metadata(dir_name).is_ok()
}

pub fn create_dir(dir_name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(dir_name)
}

