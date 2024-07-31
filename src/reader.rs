use std::{fs, io::Error};

// Read a json file from path
pub fn read_json(path: String) -> String {
    let read_result: Result<String, Error> = fs::read_to_string(path);

    let content: String = match read_result {
        Ok(file ) => file,
        Err(_) => panic!("Invalid file path or file does not exist.")
    };

    return content;
}