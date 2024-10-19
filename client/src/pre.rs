use std::io::{Read, Write, Error};
use std::path::{Path, PathBuf};
use ssh2::Session;
use std::net::TcpStream;
use rpassword::prompt_password;

use crate::utils;

// download file via ssh from server
// takes:
//   ssh session (Session)
//   remote path to read file from (String)
//   local path to write file to (String)
fn get_file(session: Session, remote_file_path: String, local_file_path: String) -> Result<(), Error> {
    // create SFTP session
    let sftp = session.sftp().expect("failed to create SFTP session");

    // open remote file
    let mut remote_file = sftp
        .open(Path::new(remote_file_path.clone().as_str()))
        .expect("failed to open remote file");

    // create local file
    let mut local_file = std::fs::File::create(local_file_path.clone().as_str())
        .expect("failed to create local file");

    // read remote file and write it to local file
    let mut buffer = Vec::new();
    remote_file.read_to_end(&mut buffer).expect("failed to read remote file");
    local_file.write_all(&buffer).expect("failed to write to local file");

    println!("file downloaded successfully");
    Ok(())
}

// get files from pre-server
// takes:
//   local path (PathBuf)
//   the ssh key path (PathBuf), ssh key isEncrypted (bool)
pub fn get_pre_files(local_path: PathBuf, ssh_private_key_tuple: (PathBuf, bool)) {
    let pre_server_json_path: &str = "hosts/pre_server.json"; // get path to pre_server.json

    // get username, host and ssh private key path
    let username: String = utils::get_from_json(local_path.join(pre_server_json_path), "username");
    println!("USERNAME: {}", username);
    let host: String = utils::get_from_json(local_path.join(pre_server_json_path), "host");
    let host_local_path: String = "/home/".to_string() + &username + "/.vote42.rs/";
    println!("HOST_LOCAL_PATH: {}", host_local_path);
    let ssh_private_key_path = ssh_private_key_tuple.0;

    // get remote path to vote template
    let vote_template_name: String = utils::get_from_json(local_path.join(pre_server_json_path), "vote_template");
    println!("vote_template_name: {}", vote_template_name);
    let vote_template_remote_path: String = host_local_path + "srv/" + &vote_template_name;
    println!("vote_template_remote_path: {}", vote_template_remote_path);
    let vote_template_local_path: String = local_path.to_string_lossy().to_string() + &vote_template_name;
    println!("vote_template_local_path: {}", vote_template_local_path);

    // create TCP connection to server
    let tcp = TcpStream::connect(host).expect("failed to connect to sever");
    
    // create a new ssh session
    let mut session = Session::new().expect("failed to create SSH session");
    session.set_tcp_stream(tcp);
    session.handshake().expect("failed to handshake");

    // check if key is password encrypted
    if ssh_private_key_tuple.1 {
        // get password for ssh key
        println!("to use the ssh key you have to enter the password");
        let ssh_private_key_password: String = prompt_password("> ").unwrap();

        // authenticate with server using key and password
        session
            .userauth_pubkey_file(username.as_str(), None, Path::new(&ssh_private_key_path), Some(ssh_private_key_password.clone().as_str()))
            .expect("authentication with password failed");
    } else {
        // authenticate with server using key without password
        session
            .userauth_pubkey_file(username.as_str(), None, Path::new(&ssh_private_key_path), None)
            .expect("authentication without password failed");
    }

    // check for successful authentication
    if !session.authenticated() {
        panic!("authentication failed");
    }

    // get files
    let _ = get_file(session, vote_template_remote_path, vote_template_local_path);
}
