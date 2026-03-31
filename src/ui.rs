pub mod ui {
    use crate::list_handler::RatedList;
    use crate::list_handler::list_handler::{list_add, list_edit, list_remove, list_to_string};
    use crate::list_handler::{self, list_handler::list_build};
    use std::io::{stdin};

    pub fn run(lists: &mut Vec<list_handler::RatedList>) {
        let mut is_running: bool = true;

        lists.push(list_build()); // FIXME: INDEX SHOULD NOT BE 0 

        while is_running  {
            print_menu();
            let input: String = get_input(String::from("What do you want to do?"));

            let command: char = parse_command(input);
            is_running = !exec_command(command, &mut lists[0]);
        }

        println!("Quitting...");
    }

    fn print_menu() {
        println!("+--------------------------------+");
        println!("| Welcome to Rated List Creator! |");
        println!("+--------------------------------+");
        println!("| Commands:                      |");
        println!("| A - Add to list                |");
        println!("| R - Remove from list           |");
        println!("| E - Edit entry in list         |");
        println!("| L - Print list                 |");
        println!("| Q - Quit                       |");
        println!("+--------------------------------+");
    }

    fn get_input(question: String) -> String {
        let mut s: String = String::new();

        println!("{}: ", question);
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        return s;
    }

    fn parse_command(input: String) -> char {
        if input.is_empty() {
            return '_';
        }

        let first_char: char = input.to_lowercase().chars().nth(0).unwrap(); 

        match first_char {
            'a' | 'r' | 'e' | 'l' | 'q' => return first_char,
            _ => return '_'
        }
    }

    fn exec_command(command: char, rl: &mut RatedList) -> bool {
        match command {
            'a' => add_to_list(rl),
            'r' => remove_from_list(rl),
            'e' => edit_list(rl),
            'l' => print_list(rl),
            'q' => return true,
            _ => return false // TODO: Should probably return an error or whatever
        };

        return false;
    }

    fn add_to_list(rl: &mut RatedList) {
        println!(">>>Adding to list<<<");
        let name: String = get_input(String::from("Enter name of entry")).trim().to_owned();
        let rating: u32 = get_input(String::from("Enter rating")).trim().parse().expect("Rating was incorrectly entered...");
        let note: String = get_input(String::from("Enter note (may leave empty)")).trim().to_owned();
        list_add(rl, name, rating, note);
    }

    fn remove_from_list(rl: &mut RatedList) {
        println!(">>>Removing from list<<<");
        let mut name: String = get_input(String::from("What entry do you want to remove?")).trim().to_owned();
        list_remove(rl, &mut name);
        return;
    }

    fn edit_list(rl: &mut RatedList) {
        let mut name: String = get_input(String::from("Enter current name of entry to edit")).trim().to_owned();
        let new_name: String = get_input(String::from("Enter new name if you wish to change it")).trim().to_owned();
        let new_rating: u32 = get_input(String::from("Enter new rating if you wish to change it")).trim().parse().expect("Rating was incorrectly entered...");
        let new_note: String = get_input(String::from("Enter new note if you wish to change it")).trim().to_owned();

        list_edit(rl, &mut name, new_name, new_rating, new_note);
    }

    fn print_list(rl: &mut RatedList) {
        println!(">>>Printing list<<<");
        println!("{}", list_to_string(rl));
        return;
    }
}