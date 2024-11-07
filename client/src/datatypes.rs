use serde::Deserialize;
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Party {
    name: String,
    long_name: String,
    color: (u8, u8, u8),
    candidates: Vec<String>,
    votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct Vote {
    election_site: String,
    election_admin: String,
    datetime: String,
    parties: Vec<Party>,
}

impl Vote {
    pub fn create_from_json(json_file_path: String) -> Result<Self> {
        println!("attempting to read JSON template file: {}", json_file_path);
        let file = File::open(json_file_path)
            .map_err(|e| format!("failed to open file: {}", e))
            .unwrap();

        let reader = BufReader::new(file);

        let vote: Vote = serde_json::from_reader(reader)?;

        println!("{:?}", vote);

        Ok(vote)
    }
}
