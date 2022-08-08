
use std::error::Error;
use std::collections::HashMap;
use std::fs;

use serde_json;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::issue;


#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub issues: HashMap<String, issue::State>,
    pub last_run: DateTime<Utc>,
}

impl State {
    fn new() -> State {
        State{
            issues: HashMap::new(),
            last_run: Utc::now(),
        }
    }

    // Decode state saved as json
    fn parse(content: &String) -> Result<State, Box<dyn Error>> {
        let state = serde_json::from_str(content)?;
        Ok(state)
    }

    // Write json representation to file
    pub fn save(&self, filename: String) -> Result<(), Box<dyn Error>> {
        let out = fs::File::create(filename)?;
        serde_json::to_writer(out, self)?;
        Ok(())
    }
}

pub fn from_file(filename: String) -> Result<State, Box<dyn Error>> {
    let file = fs::read_to_string(filename);
    match file {
        Ok(data) => State::parse(&data),
        _ => Ok(State::new())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::{Utc, TimeZone};

    use crate::state::{issue, State};

    #[test]
    fn serialize_state() {
        let mut issues = HashMap::new();
        issues.insert("clean_room.md".to_owned(), issue::State{
            is_open: false,
            last_open: Utc.timestamp(0, 0),
        });

        let state = State{
            last_run: Utc::now(),
            issues: issues,
        };

        let json = serde_json::to_string(&state).unwrap();
        println!("JSON: {}", json)
    }
}

