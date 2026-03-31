use std::collections::HashMap;
use chrono::{DateTime, Datelike, Local};

struct Entry {
    name: String,
    date: DateTime<Local>,
    rating: u32,
    note: String,
}

impl Entry {
    fn new(name: String, date: DateTime<Local>, rating: u32, note: String) -> Entry {
        return Entry {
            name,
            date,
            rating,
            note
        };
    }

    fn to_string(&mut self) -> String {
        return format!("{0}:\n    {1}\n    {2}-{3}-{4}\n    {5}\n", 
            self.name, 
            self.rating, 
            self.date.year(), 
            self.date.month(), 
            self.date.day(), 
            self.note);
    }
}

pub struct RatedList { // TODO: Should have a name
    // name: String, 
    // system: Enum 
    table: HashMap<String, Entry>,
}

impl RatedList {
    fn new() -> RatedList {
        RatedList { table: HashMap::new() }
    }

    fn add(&mut self, name: String, entry: Entry) {
        self.table.insert(name, entry); // Entry should probably be mutable
    }

    fn remove(&mut self, name: &mut String) {
        self.table.remove(name);
    }

    fn get(&mut self, name: &mut String) -> &mut Entry {
        return self.table.get_mut(name).expect("Name was not in list...");
    }

    fn to_string(&mut self) -> String {
        let mut list_str: String = String::new();

        for entry in self.table.values_mut() {
            list_str.push_str(entry.to_string().as_mut_str());
        }

        return list_str.to_owned();
    }
}

pub mod list_handler {
    use chrono::{DateTime, Local};
    use crate::list_handler::{Entry, RatedList};

    fn entry_build(name: String, date: DateTime<Local>, rating: u32, note: String) -> Entry {
        return Entry::new(name, date, rating, note);
    }

    pub fn list_build() -> RatedList {
        return RatedList::new();
    }

    pub fn list_add(rl: &mut RatedList, name: String, rating: u32, note: String) {
        let date: DateTime<Local> = Local::now();
        let new_entry: Entry = entry_build(name.clone(), date, rating, note);

        rl.add(name, new_entry);
    }

    pub fn list_remove(rl: &mut RatedList, name: &mut String) {
        rl.remove(name);
    }

    pub fn list_edit(rl: &mut RatedList, name: &mut String, new_name: String, new_rating: u32, new_note: String) {
        let to_edit: &mut Entry = rl.get(name);

        if !new_name.is_empty() {
            to_edit.name = new_name;
        }

        if new_rating != 0 {
            to_edit.rating = new_rating;
        }

        if !new_note.is_empty() {
            to_edit.note = new_note;
        }
    }

    pub fn list_to_string(rl: &mut RatedList) -> String {
        return rl.to_string();
    }
}