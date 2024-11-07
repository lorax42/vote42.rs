use serde::Deserialize;
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub election_site: String,
    pub election_admin: String,
}

impl Config {
    // popoulate Config struct from JSON file
    // takes:
    //   path to JSON file (PathBuf)
    // returns:
    //   config Struct (Config)
    pub fn create_from_json(json_file_path: PathBuf) -> Result<Self> {
        println!(
            "attempting to read JSON template file: {:?}",
            json_file_path
        );
        let file = File::open(json_file_path)
            .map_err(|e| format!("E: failed to open config file: {}", e))
            .unwrap();

        let reader = BufReader::new(file);

        let config: Config = serde_json::from_reader(reader)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
struct Party {
    name: String,
    long_name: String,
    color: (u8, u8, u8),
    candidates: Vec<String>,
    pub votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct Vote {
    pub election_site: String,
    pub election_admin: String,
    datetime: String,
    parties: Vec<Party>,
}

impl Vote {
    // popoulate Vote struct from JSON file
    // takes:
    //   path to JSON file (PathBuf)
    // returns:
    //   vote Struct (Vote)
    pub fn create_from_json(json_file_path: PathBuf) -> Result<Self> {
        println!(
            "attempting to read JSON template file: {:?}",
            json_file_path
        );
        let file = File::open(json_file_path)
            .map_err(|e| format!("E: failed to open vote template file: {}", e))
            .unwrap();

        let reader = BufReader::new(file);

        let vote: Vote = serde_json::from_reader(reader)?;

        Ok(vote)
    }
}
