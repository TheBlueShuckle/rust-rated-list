pub mod ui {
    use chrono::Local;

    use crate::list_handler::RatedList;
    use crate::list_handler::list_handler::list_add;
    use crate::list_handler::{self, list_handler::list_build};
    use std::collections::HashMap;
    use std::io::{stdin};
    use chrono::{DateTime, Utc};

    pub fn run(lists: &mut Vec<&mut list_handler::RatedList>) {
        let mut is_running: bool = true;

        let mut rl: RatedList = list_build();

        lists.insert(0, &mut rl); // FIXME: INDEX SHOULD NOT BE 0 

        while is_running  {
            print_menu();
            let mut input: String = get_input("What do you want to do?".to_owned());

            let command: char = parse_command(&mut input);
            is_running = !exec_command(command, lists[0]);
        }
    }

    fn print_menu() {
        println!("
             +--------------------------------+
             | Welcome to Rated List Creator! |
             +--------------------------------+
             | Commands:                      |
             | A - Add to list                |
             | R - Remove from list           |
             | L - Print list                 |
             | Q - Quit                       |
             +--------------------------------+"
        );
    }

    fn get_input(question: String) -> String {
        let mut s: String = String::new();

        println!("{}: ", question);
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        return s;
    }

    fn parse_command(input: &mut String) -> char {
        if input.is_empty() {
            return '_';
        }

        let first_char: char = input.chars().nth(0).unwrap(); 

        match first_char {
            'A' | 'R' | 'L' | 'Q' => return first_char,
            _ => return '_'
        }
    }

    fn exec_command(command: char, rl: &mut RatedList) -> bool {
        match command {
            'A' => add_to_list(rl),
            'R' => remove_from_list(rl),
            'L' => print_list(rl),
            'Q' => return true,
            _ => return false // TODO: Should probably return an error or whatever
        };

        return false;
    }

    fn add_to_list(rl: &mut RatedList) {
        println!("Adding to list");
        // let name: String = get_input("Enter name of entry".to_owned());
        // let date: DateTime<Local> = Local::now();
        // // let rating: u32 = get_input("Enter rating".to_owned());
        // let note: String = get_input("Enter note (may leave empty)".to_owned());
        // list_add(rl, name, date, 0, note)
    }

    fn remove_from_list(rl: &mut RatedList) {
        println!("Removing from list");
        return; // Stub
    }

    fn print_list(rl: &mut RatedList) {
        println!("Printing list");
        return; // Stub
    }
}