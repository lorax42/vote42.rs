use std::fs;
use std::fs::File;
use std::path::PathBuf;
use dirs;
use std::io::{BufReader, BufRead, Error, self};

mod utils;
mod pre;
mod datatypes;

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
            Err(e) => eprintln!("E: Error creating file: {}", e),
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
                let file_name = file_list[0].file_name();
                let file_name: &str = match file_name.to_str() {
                    Some(s) => s,
                    None => {
                        eprintln!("failed to get SSH key path from file name");
                        return None;
                    }
                };

                let file_path: PathBuf = ssh_private_key_dir.join(file_name); // get file's path
                // let file = File::open(file_path.clone()).expect("failed to open file");
                let file = match File::open(file_path.clone()){
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("failed to open file: {}", e);
                        return None;
                    }
                };
                let mut reader = BufReader::new(file);
                let mut header = String::new();
                // let num_lines = reader.read_line(&mut header).expect("failed to read first line of file");
                let num_lines = match reader.read_line(&mut header) {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read first line of file: {}", e);
                        return None;
                    }
                };

                // read forst line of file
                if num_lines > 0 {
                    // check if ssh key is encrypted or not
                    if header.contains("ENCRYPTED") {
                        return Some((file_path, true));
                    } else if header.contains("PRIVATE KEY") {
                        return Some((file_path, false));
                    } else {
                        eprintln!("E: file is not a ssh private key");
                        return None;
                    }
                } else {
                    eprintln!("E: first line is empty");
                    return None;
                }
            } else {
                eprintln!("E: there should be exactly on key in {:?}. instead there are {}", ssh_private_key_dir, file_list.len());
                eprintln!("E: make sure there is only one ssh key and that it is the right one");
                return None;
            }
        }
        Err(e) => {
            eprintln!("E: failed to read the directory '{:?}': {}", ssh_private_key_dir, e);
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
            eprintln!("E: could not find the home directory");
            return
        }
    };
    println!("HOME: {:?}", home_path);

    // get local path (~/vote42.rs/)
    let local_path: PathBuf = home_path.join(LOCAL_DIR);
    println!("LOCAL: {:?}", local_path);

    // make local directories
    match make_local_dirs(home_path.clone()) {
        Ok(_) => println!("local dirs made"),
        Err(e) => {
            eprintln!("E: failed to make local dirs: {}", e);
            return
        }
    };

    // make the config file
    match make_config(local_path.clone()) {
        Ok(_) => println!("config file has been made"),
        Err(e) => {
            eprintln!("failed to make config: {}", e);
            return
        }
    };

    // check for (single!) ssh key and get it's path and isEncrypted
    // let ssh_private_key_tuple: (PathBuf, bool) = check_ssh_private_key(local_path.clone()).expect("could not get SSH private key path");
    let ssh_private_key_tuple: (PathBuf, bool) = match check_ssh_private_key(local_path.clone()) {
        Some(t) => t,
        None => {
            eprintln!("failed to get ssh private key tuple");
            return
        }
    };

    // get files from pre-server
    let vote_template_local_path: String = String::from("/home/lorax/.vote42.rs/vote_template.json"); // DEBUG
    /* let vote_template_local_path: String = match pre::get_pre_files(local_path.clone(), ssh_private_key_tuple.clone()) {
        Ok(s) => {
            println!("pre files have been received");
            s
        },
        Err(e) => {
            eprintln!("failed to get pre files: {}", e);
            return
        }
    }; */

    let vote: datatypes::Vote = match datatypes::Vote::create_from_json(vote_template_local_path) {
        Ok(v) => v,
        Err(e) => {
            println!("failed to parse JSON to Vote struct: {}", e);
            return;
        }
    };
    println!("{:?}", vote);
}
