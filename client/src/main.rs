use std::fs;
use std::fs::File;
use std::path::PathBuf;
use dirs;
use std::io::{BufReader, BufRead, Error, self};

mod utils;
mod pre;
// mod datatypes;

const LOCAL_DIR: &str = ".vote42.rs/"; // name of local dir
const SSH_LOCAL_DIR: &str = "ssh/"; // local dir for ssh stuff
const CONFIG: &str = "config.json"; // name of config file in local directory

// make local directories
// takes:
//   $HOME (PathBuf)
fn make_local_dirs(home_path: PathBuf) -> Result<(), Error> {
    // local dirs
    let local_dirs = vec![
        LOCAL_DIR.to_string(),
        LOCAL_DIR.to_string() + SSH_LOCAL_DIR,
    ];

    // make all dirs in vec
    for dir in local_dirs {
        utils::make_local_dir(home_path.clone(), dir.to_string())?;
    }

    Ok(())
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
// takes:
//   local path (PathBuf)
fn make_config(local_path: PathBuf) -> Result<(), Error> {
    // check if config exists
    if !utils::check_file(local_path.clone(), CONFIG.to_string()) {
        let mut config_file_src = File::open(CONFIG)?; // src config file

        // dest config file
        let mut config_file_dest = File::create(local_path.join(CONFIG))?;

        // copy src file to dest file
        io::copy(&mut config_file_src, &mut config_file_dest)?;

        println!("file copied successfully: {:?} to {:?}", config_file_src, config_file_dest);
    }

    Ok(())
}

// check if ssh key is available and if it is encrypted or not
// takes:
//   path to local directory (~/.vote42.rs/)
// returns:
//   path to file (PathBuf)
//   isEncrypted (bool)
fn check_ssh_private_key(local_path: PathBuf) -> Option<(PathBuf, bool)> {
    let ssh_private_key_dir = local_path.join(SSH_LOCAL_DIR); // program's local ssh directory

    // check if there are files in ssh dir
    match fs::read_dir(ssh_private_key_dir.clone()) {
        Ok(entries) => {
            // Count the number of files in the directory
            let file_list: Vec<_> = entries
                .filter_map(Result::ok) // Filter out any errors
                .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)) // Check if it's a file
                .collect();

            // Check if there is exactly one file
            if file_list.len() == 1 {
                // get file's name
                let file_name_bind = file_list[0].file_name();
                let file_name: &str = file_name_bind.to_str().expect("failed to get SSH key path from file name");

                let file_path: PathBuf = ssh_private_key_dir.join(file_name); // get file's path
                let file = File::open(file_path.clone()).expect("failed to open file");
                let mut reader = BufReader::new(file);
                let mut header = String::new();

                // read forst line of file
                if reader.read_line(&mut header).expect("failed to read first line of file") > 0 {
                    // check if ssh key is encrypted or not
                    if header.contains("ENCRYPTED") {
                        return Some((file_path, true));
                    } else if header.contains("PRIVATE KEY") {
                        return Some((file_path, false));
                    } else {
                        eprintln!("file is not a ssh private key");
                        return None;
                    }
                } else {
                    eprintln!("first line is empty");
                    return None;
                }
            } else {
                eprintln!("there should be exactly on key in {:?}. instead there are {}", ssh_private_key_dir, file_list.len());
                eprintln!("make sure there is only one ssh key and that it is the right one");
                return None;
            }
        }
        Err(e) => {
            eprintln!("failed to read the directory '{:?}': {}", ssh_private_key_dir, e);
            return None;
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

    // get local path (~/vote42.rs/)
    let local_path: PathBuf = home_path.join(LOCAL_DIR);
    println!("LOCAL: {:?}", local_path);

    let _ = make_local_dirs(home_path.clone()); // make local directories
    let _ = make_config(local_path.clone()); // make the config file

    // check for (single!) ssh key and get it's path and isEncrypted
    let ssh_private_key_tuple: (PathBuf, bool) = check_ssh_private_key(local_path.clone()).expect("could not get SSH private key path");

    let _ = pre::get_pre_files(local_path.clone(), ssh_private_key_tuple.clone()); // get files from pre-server
}
