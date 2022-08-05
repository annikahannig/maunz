
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::issue;


#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub issues: HashMap<String, issue::State>,
    pub last_run: DateTime<Utc>,
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

