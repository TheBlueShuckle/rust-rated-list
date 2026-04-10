use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RatingSystem {
    FiveStars,
    TenStars,
    TenHalfStars,
}

impl Clone for RatingSystem {
    fn clone(&self) -> Self {
        match self {
            RatingSystem::FiveStars => RatingSystem::FiveStars,
            RatingSystem::TenStars => RatingSystem::TenStars,
            RatingSystem::TenHalfStars => RatingSystem::TenHalfStars,
        }
    }
}

impl RatingSystem {
    fn rating_to_string(&self, rating: u32) -> String {
        let mut string_rating: String = String::new();

        match self {
            RatingSystem::FiveStars => {
                let mut count: u32 = 0;

                while count < 5 {
                    if count >= rating {
                        string_rating.push_str("☆ ");
                    }

                    else {
                        string_rating.push_str("★ ");
                    }

                    count += 1;
                }

                return string_rating;
            }

            RatingSystem::TenStars => {
                let mut count: u32 = 0;

                while count < 10 {
                    if count >= rating {
                        string_rating.push_str("☆ ");
                    }

                    else {
                        string_rating.push_str("★ ");
                    }

                    count += 1;
                }

                return string_rating;
            }
            RatingSystem::TenHalfStars => {
                let mut count: u32 = 0;

                while count < 10 {
                    if count >= rating {
                        string_rating.push_str("☆ ");
                    }

                    else if rating % 2 != 0 && count + 2 > rating {
                        string_rating.push_str("½ ");
                    }

                    else {
                        string_rating.push_str("★ ");
                    }

                    count += 2;
                }

                return string_rating;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    date: String,
    rating: u32,
    system: RatingSystem,
    note: String,
}

impl Entry {
    fn new(
        name: String, 
        date: String, 
        rating: u32, 
        system: RatingSystem, 
        note: String
    ) -> Entry {
        return Entry {
            name,
            date,
            rating,
            system,
            note
        };
    }

    fn to_string(&mut self) -> String {
        return format!("{0}:\n    {1}\n    {2}\n    {3}\n", 
            self.name, 
            self.system.rating_to_string(self.rating),
            self.date, 
            self.note);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatedList { 
    name: String, 
    system: RatingSystem,
    table: HashMap<String, Entry>,
}

impl RatedList {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn new(name: String, system: RatingSystem) -> RatedList {
        RatedList { name, system, table: HashMap::new() }
    }

    fn add(&mut self, name: String, entry: Entry) {
        self.table.insert(name, entry);
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
    use std::{fs::{self, File}, io::{BufReader, Read}};
    use chrono::{Local};
    use crate::list_handler::{Entry, RatedList, RatingSystem};

    fn entry_build(
        name: String, 
        date: String, 
        rating: u32, 
        system: RatingSystem, 
        note: String
    ) -> Entry {
        return Entry::new(name, date, rating, system, note);
    }

    pub fn list_build(name: String, system: RatingSystem) -> RatedList {
        return RatedList::new(name, system);
    }

    pub fn list_add(rl: &mut RatedList, name: String, rating: u32, note: String) {
        let date_str: String = Local::now().date_naive().to_string();
        let new_entry: Entry = entry_build(
                                name.clone(), 
                                date_str, 
                                rating, 
                                rl.system.clone(),
                                note);

        rl.add(name, new_entry);
    }

    pub fn list_remove(rl: &mut RatedList, name: &mut String) {
        rl.remove(name);
    }

    pub fn list_edit(
        rl: &mut RatedList, 
        name: &mut String, 
        new_name: String, 
        new_date: String,
        new_rating: u32, 
        new_note: String
    ) {
        let old_entry: &mut Entry = rl.get(name);
        let new_entry: Entry = entry_build(
            if !new_name.is_empty() { new_name } else { old_entry.name.clone() }, 
            if !new_date.is_empty() { new_date } else { old_entry.date.clone() }, 
            if new_rating != 0 { new_rating } else { old_entry.rating }, 
            old_entry.system.clone(), 
            if !new_note.is_empty() { new_note } else { old_entry.note.clone() });

        rl.remove(name);
        rl.add(new_entry.name.clone(), new_entry);
    }

    pub fn list_update_rating(rl: &mut RatedList, name: &mut String, new_rating: u32) {
        let to_edit: &mut Entry = rl.get(name);
        to_edit.rating = new_rating;
    }

    pub fn list_to_string(rl: &mut RatedList) -> String {
        return rl.to_string();
    }

    pub fn list_save(rl: &mut RatedList) -> std::io::Result<()> {
        let path: String = String::from("lists/");
        let file_path: String = format!("{0}{1}", path, rl.get_name());

        let serialized: String = serde_json::to_string(&rl).unwrap();

        return fs::write(file_path, serialized);
    }

    pub fn list_load(name: String) -> RatedList {
        let path: String = String::from("lists/");
        let file_path: String = format!("{0}{1}", path, name);
        let f: File = File::open(file_path).expect("!!!ERROR IN list_load!!!");
        let mut reader: BufReader<File> = BufReader::new(f);
        
        let mut deserialized: String = String::new();

        reader.read_to_string(&mut deserialized).expect("ERROR");

        let rl: RatedList = serde_json::from_str(&deserialized.as_str()).unwrap();

        return rl;
    }
}