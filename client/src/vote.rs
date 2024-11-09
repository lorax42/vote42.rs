use std::io::{stdin, Error};

use crate::datatypes;

fn set_vote(party: &mut datatypes::Party) -> Result<(), Error> {
    println!("Enter {}'s votes", party.get_name());

    let mut num = String::new();
    stdin().read_line(&mut num).expect("E: failed to read line");

    let num: u32 = num.trim().parse().expect("E: failed to parse string");
    party.set_votes(num);

    Ok(())
}

pub fn set_votes(vote: &mut datatypes::Vote) -> Result<(), Error> {
    for party in vote.get_mut_parties() {
        match set_vote(party) {
            Ok(_) => println!("{} vote set to {}", party.get_name(), party.get_votes()),
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(())
}
