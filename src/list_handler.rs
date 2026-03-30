use std::collections::HashMap;
use chrono::{DateTime, Local};

struct Entry {
    name: String,
    date: DateTime<Local>,
    rating: u32,
    note: String,
}

pub struct RatedList { // TODO: Should have a name
    // name: String, 
    table: HashMap<String, Entry>,
}

impl RatedList {
    fn add(&mut self, name: String, entry: Entry) {
        self.table.insert(name, entry); // Entry should probably be mutable
    }

    fn remove(&mut self, name: &mut String) {
        self.table.remove(name);
    }
}

pub mod list_handler {
    use std::{collections::HashMap};
    use chrono::{DateTime, Local};
    use crate::list_handler::{Entry, RatedList};

    fn entry_build(name: String, date: DateTime<Local>, rating: u32, note: String) -> Entry {
        return Entry {
            name,
            date,
            rating,
            note
        };
    }

    pub fn list_build() -> RatedList {
        return RatedList { 
            table: (HashMap::new()) 
        };
    }

    pub fn list_add(rl: &mut RatedList, name: String, date: DateTime<Local>, rating: u32, note: String) {
        let new_entry = entry_build(name.clone(), date, rating, note);

        rl.add(name, new_entry);
    }

    pub fn list_remove(rl: &mut RatedList, name: &mut String) {
        rl.remove(name);
    }
}