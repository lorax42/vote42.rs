use serde_json::{Result, Value};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// make a local directory based on name
// takes:
//   $HOME (PathBuf)
//   directory name (String)
pub fn make_local_dir(home_path: PathBuf, dir_name: String) -> Result<()> {
    let dir_path = home_path.join(dir_name);

    // check if directory exists
    if !dir_path.exists() {
        // create directory
        match fs::create_dir(&dir_path) {
            Ok(_) => println!("directory created: {:?}", dir_path),
            Err(e) => eprintln!("failed to create directory: {}", e),
        }
    } else {
        println!("directory already exists: {:?}", dir_path);
    }
    Ok(())
}

// check if file exists
// takes:
//   $HOME (PathBuf)
//   name of file (String)
// returns:
//   doesExists (bool)
pub fn check_file(file_path: PathBuf) -> bool {
    // check if file exists
    match fs::metadata(&file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("file exists: {:?}", file_path);
            } else {
                println!("path exists, but it is not a file: {:?}", file_path);
            }

            return true;
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("file does not exist: {:?}", file_path);
            } else {
                eprintln!("error checking file: {}", e);
            }

            return false;
        }
    }
}

// get a value from a JSON file
// takes:
//   path to file (PathBuf)
//   key to look for in JSON file (&str)
// returns:
//   the value of the key in the JSON file (String)
pub fn get_from_json(file_path: PathBuf, key: &str) -> String {
    let file = File::open(file_path).expect("E: unable to open file");
    let reader = BufReader::new(file);

    // Deserialize the JSON into a serde_json::Value
    let json_data: Value = serde_json::from_reader(reader).expect("E: failed to deserialize JSON");

    let data_string: String = json_data
        .get(key)
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(i.to_string())
                } else if let Some(f) = n.as_f64() {
                    Some(f.to_string())
                } else {
                    None
                }
            }
            Value::Bool(b) => Some(b.to_string()),
            Value::Array(arr) => Some(format!("{:?}", arr)),
            Value::Object(obj) => Some(format!("{:?}", obj)),
            Value::Null => {
                panic!("E: field {} not found", key);
                // None
            }
        })
        .unwrap();

    data_string
}
