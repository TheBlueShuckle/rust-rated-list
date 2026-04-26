use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    date: String,
    note: String,
}

impl Entry {
    fn new(
        name: String, 
        date: String, 
        note: String
    ) -> Entry {
        return Entry {
            name,
            date,
            note
        };
    }

    fn to_string(&self) -> String {
        return format!("{0}:  \n>    {1}  \n>    {2}  \n", 
            self.name, 
            self.date, 
            self.note);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchList { 
    name: String, 
    table: HashMap<String, Entry>,
}

impl WatchList {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn new(name: String) -> WatchList {
        return WatchList { name, table: HashMap::new() }
    }

    fn add(&mut self, name: String, entry: Entry) {
        self.table.insert(name, entry);
    }

    fn remove(&mut self, name: &mut String) {
        self.table.remove(name);
    }

    fn get(&mut self, name: &String) -> &mut Entry {
        return self.table.get_mut(name).expect("Name was not in list...");
    }

    pub fn entry_names(&self) -> Vec<&String> {
        return self.table.keys().collect();
    }

    pub fn entry_to_string(&self, name: &String) -> String {
        return self.table.get(name).expect("Name was not in hashmap").to_string();
    }

    fn to_string(&mut self) -> String {
        let mut list_str: String = String::new();

        for entry in self.table.values_mut() {
            list_str.push_str(entry.to_string().as_mut_str());
        }

        return list_str.to_owned();
    }
}

pub mod watchlist_handler {
    use std::{
        fs::{
            self, 
            File
        }, 
        io::{
            BufReader, 
            Read
        }
    };
    use chrono::{
        Local
    };
    use crate::watchlist_handler::{
        Entry, 
        WatchList, 
    };

    fn entry_build(
        name: String, 
        date: String, 
        note: String
    ) -> Entry {
        return Entry::new(name, date, note);
    }

    pub fn watchlist_build(name: String) -> WatchList {
        return WatchList::new(name);
    }

    pub fn watchlist_add(wl: &mut WatchList, name: String, note: String) {
        let date_str: String = Local::now().date_naive().to_string();
        let new_entry: Entry = entry_build(
                                name.clone(), 
                                date_str, 
                                note);

        wl.add(name, new_entry);
    }

    pub fn watchlist_remove(wl: &mut WatchList, name: &mut String) {
        wl.remove(name);
    }

    pub fn watchlist_edit(
        wl: &mut WatchList, 
        name: &mut String, 
        new_name: String, 
        new_date: String,
        new_note: String
    ) {
        let old_entry: &mut Entry = wl.get(name);
        let new_entry: Entry = entry_build(
            if !new_name.is_empty() { new_name } else { old_entry.name.clone() }, 
            if !new_date.is_empty() { new_date } else { old_entry.date.clone() }, 
            if !new_note.is_empty() { new_note } else { old_entry.note.clone() });

        wl.remove(name);
        wl.add(new_entry.name.clone(), new_entry);
    }

    pub fn watchlist_to_string(rl: &mut WatchList) -> String {
        return rl.to_string();
    }

    pub fn watchlist_save(wl: &mut WatchList) -> std::io::Result<()> {
        let path: &str = "./lists";
        let dir_path: String = format!("{0}/{1}", path, wl.get_name().to_lowercase()); // Ex lists/albums
        let file_path: String = format!("{0}/{1}-watchlist", dir_path, wl.get_name().to_lowercase()); // Ex lists/albums/Albums

        let serialized: String = serde_json::to_string(&wl).unwrap();

        println!("{}", dir_path);

        match fs::create_dir_all(dir_path) {
            Ok(_) => {println!("success")},
            Err(_m) => {println!("fail :(: {}", _m)},
        }
        return fs::write(file_path, serialized);
    }

    pub fn watchlist_load_or_create(list_name: String) -> WatchList {
        let path: &str = "./lists";
        let dir_path: String = format!("{0}/{1}", path, list_name.to_lowercase());
        let main_file_path: String = format!("{0}/{1}-watchlist", dir_path, list_name.to_lowercase());
        let file: File;

        match File::open(main_file_path) {
            Ok(f) => file = f,
            Err(_) => return watchlist_build(list_name),
        }

        let mut reader: BufReader<File> = BufReader::new(file);
        
        let mut deserialized: String = String::new();

        reader.read_to_string(&mut deserialized).expect("ERROR");

        let wl: WatchList = serde_json::from_str(&deserialized.as_str()).unwrap();

        return wl;
    }
}