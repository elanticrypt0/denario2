use std::{env, fs};

// Create a dir in
pub fn mk_dir_in_current_folder(new_dir:String){
    let mut path = env::current_dir().unwrap();
    path.push(new_dir);
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

// create file in
pub fn create_file_in_current_folder(new_file:String){
    let mut path = env::current_dir().unwrap();
    path.push(new_file);
    if !path.exists() {
        fs::write(path, "").unwrap();
    }
}