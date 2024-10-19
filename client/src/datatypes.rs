use serde::Deserialize;
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Party {
    name: String,
    long_name: String,
    color: (u8, u8, u8),
    candiates: Vec<String>,
    votes: u32,
}

#[derive(Debug, Deserialize)]
struct Vote {
    election_site: String,
    election_admin: String,
    datetime: String,
    parties: Vec<Party>,
}

impl Vote {
    fn create_from_json(json_file_path: String) -> Self {
        let file = File::open(json_file_path).expect("failed to open file");
        let reader = BufReader::new(file).expect("failed to create reader");

        let vote: Vote = serde_json::from_reader(reader).expect("failed to parse json to data type Vote");

        println!("{}", vote);

        vote
    }
}
