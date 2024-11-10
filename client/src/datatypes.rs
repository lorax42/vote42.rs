use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use chrono::Local;

#[derive(Debug, Deserialize)]
pub struct Config {
    election_site: String,
    election_admin: String,
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

    // get election site name
    // takes:
    //   config (Config)
    // returns:
    //   name of election site (String)
    pub fn get_election_site(&self) -> String {
        self.election_site.clone()
    }

    // get election admin name
    // takes:
    //   config (Config)
    // returns:
    //   name of election admin (String)
    pub fn get_election_admin(&self) -> String {
        self.election_admin.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color (u8, u8, u8);

impl Clone for Color {
    fn clone(&self) -> Self {
        Color (self.0.clone(), self.1.clone(), self.2.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    name: String,
    long_name: String,
    color: Color,
    candidates: Vec<String>,
    votes: u32,
}

impl Clone for Party {
    fn clone(&self) -> Self {
        Party {
            name: self.name.clone(),
            long_name: self.long_name.clone(),
            color: self.color.clone(),
            candidates: self.candidates.clone(),
            votes: self.votes.clone()
        }
    }
}

impl Party {
    // set the votes of the party
    // takes:
    //   num of votes (u32)
    //   mutable reference to party struct (&mut Party)
    pub fn set_votes(&mut self, votes: u32) {
        self.votes = votes;
    }

    // get party name
    // takes:
    //   party (Party)
    // returns:
    //   party name (String)
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // get party long name
    // takes:
    //   party (Party)
    // returns:
    //   party long name (String)
    pub fn get_long_name(&self) -> String {
        self.long_name.clone()
    }

    // get party color
    // takes:
    //   party (Party)
    // returns:
    //   party color (Color)
    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    // get party candidates
    // takes:
    //   party (Party)
    // returns:
    //   party candidates (Vec<String>)
    pub fn get_candidates(&self) -> Vec<String> {
        self.candidates.clone()
    }

    // get party votes
    // takes:
    //   to party (Party)
    // returns:
    //   party votes (u32)
    pub fn get_votes(&self) -> u32 {
        self.votes.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    election_site: String,
    election_admin: String,
    datetime: String,
    parties: Vec<Party>,
}

impl Clone for Vote {
    fn clone(&self) -> Self {
        Vote {
            election_site: self.election_site.clone(),
            election_admin: self.election_admin.clone(),
            datetime: self.datetime.clone(),
            parties: self.parties.clone()
        }
    }
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

    // write to JSON file
    // takes:
    //   path to JSON file
    pub fn write_to_json(&self, mut json_file_path: PathBuf) -> Result<()> {

        if json_file_path.extension().is_none() {
            json_file_path.set_extension("json");
        }

        println!(
            "attempting to write Vote to JSON file: {:?}",
            json_file_path
        );
        let mut file = File::create(json_file_path)
            .map_err(|e| format!("E: failed to create vote file: {}", e))
            .unwrap();

        let json_string = serde_json::to_string_pretty(&self).unwrap();

        file.write_all(json_string.as_bytes())
            .map_err(|e| format!("E: failed to write JSON data to file: {}", e))
            .unwrap();

        Ok(())
    }
    
    // get vote election site name
    // takes:
    //   reference to vote (&Vote)
    // returns:
    //   election site name (String)
    pub fn get_election_site(&self) -> String {
        self.election_site.clone()
    }

    // get vote election admin name
    // takes:
    //   reference to vote (&Vote)
    // returns:
    //   election admin name (String)
    pub fn get_election_admin(&self) -> String {
        self.election_admin.clone()
    }

    // get vote datetime
    // takes:
    //   reference to vote (&Vote)
    // returns:
    //   datetime (String)
    pub fn get_datetime(&self) -> String {
        self.datetime.clone()
    }

    // get vote parties
    // takes:
    //   reference to vote (&Vote)
    // returns:
    //   parties vec (Vec<Party>)
    pub fn get_parties(&self) -> Vec<Party> {
        self.parties.clone()
    }

    // get mutable reference to vote parties
    // takes:
    //   mutable reference to vote (&mut Vote)
    // returns:
    //   mutable reference to parties vec (&mut Vec<Party>)
    pub fn get_mut_parties(&mut self) -> &mut Vec<Party> {
        &mut self.parties
    }

    // set the election site name
    // takes:
    //   name of site (String)
    //   mutable reference to vote struct (&mut Vote)
    pub fn set_election_site(&mut self, name: String) {
        self.election_site = name;
    }

    // set the election admin name
    // takes:
    //   name of site (String)
    //   mutable reference to vote struct (&mut Vote)
    pub fn set_election_admin(&mut self, name: String) {
        self.election_admin = name;
    }

    // set the votes datetime to the current local time
    // using the RFC3339 format
    // takes:
    //   mutable reference to vote struct (&mut Vote)
    pub fn set_datetime(&mut self) {
        self.datetime = Local::now().to_rfc3339();
    }
}
