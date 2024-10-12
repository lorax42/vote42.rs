use std::fs;
use std::fs::File;
use std::path::PathBuf;
use dirs;
use std::io::{Error, self};

const LOCAL_DIR: &str = ".vote42.rs/"; // name of local dir
const SSH_LOCAL_DIR: &str = "ssh/"; // local dir for ssh stuff
const CONFIG: &str = "config"; // name of config file in local directory

// make a local directory based on name
fn make_local_dir(home_path: PathBuf, dir_name: String) -> Result<(), Error> {
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

// make local directories
fn make_local_dirs(home_path: PathBuf) -> Result<(), Error> {
    // local dirs
    let local_dirs = vec![
        LOCAL_DIR.to_string(),
        LOCAL_DIR.to_string() + SSH_LOCAL_DIR,
    ];

    // make all dirs in vec
    for dir in local_dirs {
        make_local_dir(home_path.clone(), dir.to_string())?;
    }
    Ok(())
}

// check if file exists
fn check_file(home_path: PathBuf, file_name: String) -> bool {
    // file to be checked
    let file_path = home_path.join(file_name);

    // check if file exists
    match fs::metadata(&file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("File exists: {:?}", file_path);
            } else {
                println!("Path exists, but it is not a file: {:?}", file_path);
            }

            return true;
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("File does not exist: {:?}", file_path);
            } else {
                eprintln!("Error checking file: {}", e);
            }

            return false;
        }
    }
}

// make file in LOCAL_DIR
/* fn make_local_file(home_path: PathBuf, file_name: String) -> Result<(), Error> {
    let file_path = home_path.join(file_name);

    // if file doesn't exists create it
    if !file_path.exists() {
        match File::create(file_path) {
            Ok(_) => println!("File created successfully."),
            Err(e) => eprintln!("Error creating file: {}", e),
        }
    } else {
        println!("File already exists: {:?}", file_path);
    }
    Ok(())
} */

// make config file if it doesn't exist
fn make_config(local_path: PathBuf) -> Result<(), Error> {
    // check if config exists
    if !check_file(local_path.clone(), CONFIG.to_string()) {
        let mut config_file_src = File::open(CONFIG)?; // src config file

        // dest config file
        let mut config_file_dest = File::create(local_path.join(CONFIG))?;

        // copy src file to dest file
        io::copy(&mut config_file_src, &mut config_file_dest)?;

        println!("file copied successfully: {:?} to {:?}", config_file_src, config_file_dest);
    }

    Ok(())
}

// check if ssh key is available
fn check_ssh_key(local_path: PathBuf) -> String {
    let ssh_key_dir = local_path.join(SSH_LOCAL_DIR);

    match fs::read_dir(ssh_key_dir.clone()) {
        Ok(entries) => {
            // Count the number of files in the directory
            let file_list: Vec<_> = entries
                .filter_map(Result::ok) // Filter out any errors
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)) // Check if it's a file
                .collect();

            // Check if there is exactly one file
            if file_list.len() == 1 {
                let file_name = file_list[0].file_name();
                return file_name.to_string_lossy().into_owned();
            } else {
                println!("there should be exactly on key in {:?}. instead there are {}", ssh_key_dir, file_list.len());
                println!("make sure there is only one ssh key and that it is the right one");
                return "".to_string();
            }
        }
        Err(e) => {
            println!("failed to read the directory '{:?}': {}", ssh_key_dir, e);
            return "".to_string();
        }
    }
}

// driver
fn main() {
    // get home directory
    let home_path: PathBuf = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("could not find the home directory");
            return
        }
    };
    println!("HOME: {:?}", home_path);

    let local_path: PathBuf = home_path.join(LOCAL_DIR);
    println!("LOCAL: {:?}", local_path);

    let _ = make_local_dirs(home_path.clone()); // make local directories
    let _ = make_config(local_path.clone()); // make the config file

    // check for (single!) ssh key
    let ssh_key = check_ssh_key(local_path.clone());
    if ssh_key == "" {
        return
    }
}
