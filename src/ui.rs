pub mod ui {
    use chrono::NaiveDate;

    use crate::list_handler::{
        RatedList,
        RatingSystem,
    };
    use crate::list_handler::list_handler::{
        list_add,
        list_remove,
        list_edit,
        list_update_rating,
        list_build,
        list_load,
        list_save,
        list_to_string,
    };
    use crate::watchlist_handler::WatchList;
    use crate::watchlist_handler::watchlist_handler::{
        watchlist_add,
        watchlist_remove,
        watchlist_edit,
        watchlist_to_string,
        watchlist_save,
        watchlist_load_or_create,
    };
    use crate::list_exporter::list_exporter::{
        export,
    };
    use std::fs::{
        self, 
        ReadDir,
    };
    use std::io::{
        stdin,
    };

    pub fn run() {
        const LISTS_PATH: &str = "./lists/";
        let mut lists: Vec<RatedList> = load_lists(LISTS_PATH);

        main_loop(&mut lists);

        println!("Quitting...");
    }

    /* +-----------------------------------------------------------------------------------------+*/
    /* |                                   Main loop functions                                   |*/
    /* +-----------------------------------------------------------------------------------------+*/

    fn main_loop(lists: &mut Vec<RatedList>) {
        let mut is_running: bool = true;

        while is_running  {
            print_main_menu(&lists);
            let input: String = get_input("What do you want to do?");

            let command: char = parse_command_main(input);
            is_running = !exec_command_main(command, lists);
        }
    }

    fn print_main_menu(lists: &Vec<RatedList>) {
        println!("+--------------------------------+");
        println!("| Welcome to Rated List Creator! |");
        println!("+--------------------------------+");
        println!("| Below are all saved lists. If  |");
        println!("| you wish to create a new list  |");
        println!("| enter 'N', otherwise type the  |");
        println!("| number corresponding to the    |");
        println!("| list to load and edit it.      |");
        println!("| To quit, type 'Q'.             |");
        println!("+--------------------------------+");

        let mut i: u32 = 1;

        for rl in lists {
            println!("{0}. {1}", i, rl.get_name());
            i += 1;
        }
    }

    fn parse_command_main(input: String) -> char {
        if input.is_empty() {
            return '_';
        }

        let first_char: char = input.to_lowercase()
                                    .chars()
                                    .nth(0)
                                    .unwrap(); 
        
        match first_char {
            'n' | 'q' => return first_char,
            _ => {
                if first_char.is_digit(10) {
                    return first_char;
                }

                return '_';
            }
        }
    }

    fn exec_command_main(command: char, lists: &mut Vec<RatedList>) -> bool {
        match command {
            'n' => create_new_list(lists),
            'q' => return true,
            _ => {
                if command.is_digit(10) {
                    let i: usize = command.to_digit(10).unwrap() as usize;

                    if i > 0 && i <= lists.len() {
                        list_loop(&mut lists[i - 1]);
                    }
                }

                return false; // TODO: Should probably return an error or whatever
            }
        };

        return false;
    }

    /* +-----------------------------------------------------------------------------------------+*/
    /* |                                   List loop functions                                   |*/
    /* +-----------------------------------------------------------------------------------------+*/

    fn list_loop(rl: &mut RatedList) {
        let mut is_running: bool = true;

        while is_running {
            print_list_menu();
            let input: String = get_input("What do you want to do?");

            let command: char = parse_command_list(input);
            is_running = !exec_command_list(command, rl);
        }
    }

    fn print_list_menu() {
        println!("+--------------------------------+");
        println!("| Welcome to Rated List Creator! |");
        println!("+--------------------------------+");
        println!("| Commands:                      |");
        println!("| A - Add to list                |");
        println!("| R - Remove from list           |");
        println!("| E - Edit entry in list         |");
        println!("| U - Update Rating              |");
        println!("| P - Print list                 |");
        println!("| S - Save list                  |");
        println!("| X - Export list to md file     |");
        println!("| L - See watchlist              |");
        println!("| W - Go to watchlist            |");
        println!("| Q - Quit                       |");
        println!("+--------------------------------+");
    }

    fn parse_command_list(input: String) -> char {
        if input.is_empty() {
            return '_';
        }

        let first_char: char = input.to_lowercase().chars().nth(0).unwrap(); 

        match first_char {
            'a' | 'r' | 'e' | 'u'| 'p' | 's' | 'x' | 'l' | 'w' | 'q' => return first_char,
            _ => return '_'
        }
    }

    fn exec_command_list(command: char, rl: &mut RatedList) -> bool {
        match command {
            'a' => add_to_list(rl),
            'r' => remove_from_list(rl),
            'e' => edit_list(rl),
            'u' => update_rating(rl),
            'p' => print_list(rl),
            's' => save_list(rl),
            'x' => export_list(rl),
            'l' => print_wl_from_rl(rl),
            'w' => watchlist_loop(rl),
            'q' => return true,
            _ => return false // TODO: Should probably print an error or whatever
        };

        return false;
    }

    /* +-----------------------------------------------------------------------------------------+*/
    /* |                                 Watchlist loop functions                                |*/
    /* +-----------------------------------------------------------------------------------------+*/

    fn watchlist_loop(rl: &mut RatedList) {
        let mut is_running: bool = true;

        let mut wl: WatchList = watchlist_load_or_create(rl.get_name());

        while is_running {
            print_watchlist_menu();
            let input: String = get_input("What do you want to do?");

            let command: char = parse_command_watchlist(input);
            is_running = !exec_command_watchlist(command, &mut wl);
        }
    }

    fn print_watchlist_menu() {
        println!("+--------------------------------+");
        println!("| Welcome to the watchlist editor|");
        println!("+--------------------------------+");
        println!("| Commands:                      |");
        println!("| A - Add to watchlist           |");
        println!("| R - Remove from watchlist      |");
        println!("| E - Edit entry in watchlist    |");
        println!("| L - Print watchlist            |");
        println!("| S - Save watchlist             |");
        println!("| Q - Quit                       |");
        println!("+--------------------------------+");
    }

    fn parse_command_watchlist(input: String) -> char {
        if input.is_empty() {
            return '_';
        }

        let first_char: char = input.to_lowercase().chars().nth(0).unwrap(); 

        match first_char {
            'a' | 'r' | 'e' | 'l' | 's' | 'q' => return first_char,
            _ => return '_'
        }
    }

    fn exec_command_watchlist(command: char, wl: &mut WatchList) -> bool {
        match command {
            'a' => add_to_watchlist(wl),
            'r' => remove_from_watchlist(wl),
            'e' => edit_watchlist(wl),
            'l' => print_watchlist(wl),
            's' => save_watchlist(wl),
            'q' => return true,
            _ => return false // TODO: Should probably return an error or whatever
        };

        return false;
    }
    
    /* +-----------------------------------------------------------------------------------------+*/
    /* |                                     Other functions                                     |*/
    /* +-----------------------------------------------------------------------------------------+*/

    fn get_input(question: &str) -> String {
        let mut s: String = String::new();

        println!("{}: ", question);
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        return s;
    }

    // List functions

    fn create_new_list(lists: &mut Vec<RatedList>) {
        let name = get_input("Enter name of new list")
                                .trim()
                                .to_owned();
        lists.push(list_build(name, RatingSystem::TenHalfStars));
    }

    fn load_lists(path: &str) -> Vec<RatedList> {
        let mut lists: Vec<RatedList> = Vec::new();
        let paths: ReadDir = fs::read_dir(path).unwrap();

        for list_path in paths {
            let file_name: String = list_path.unwrap().file_name().display().to_string();

            if file_name == ".gitignore" { // Skip the gitignore; theres probably a better way of doing this or whatever
                continue;
            }

            println!("{}", file_name);
            lists.push(list_load(file_name));
        }

        return lists;
    }

    fn add_to_list(rl: &mut RatedList) {
        println!(">>>Adding to list<<<");
        let name: String = get_input("Enter name of entry")
                                .trim()
                                .to_owned();
        let rating: u32 = get_input("Enter rating")
                                .trim()
                                .parse()
                                .expect("Rating was incorrectly entered...");
        let note: String = get_input("Enter note (may leave empty)")
                                .trim()
                                .to_owned();
        list_add(rl, name, rating, note);
    }

    fn remove_from_list(rl: &mut RatedList) {
        println!(">>>Removing from list<<<");
        let mut name: String = get_input("What entry do you want to remove?")
                                    .trim()
                                    .to_owned();
        list_remove(rl, &mut name);
        return;
    }

    fn edit_list(rl: &mut RatedList) {
        let mut name: String = get_input("Enter current name of entry to edit")
                                    .trim()
                                    .to_owned();
        let new_name: String = get_input("Enter new name if you wish to change it")
                                    .trim()
                                    .to_owned();


        let mut new_date: String = String::new();
        let change_date: String = get_input("Do you wish to edit the date?, y for yes");
        if change_date.to_lowercase().chars().nth(0).unwrap() == 'y' {
            let datestring: String = get_date_from_input().trim().to_string();
            println!("{}", datestring);
            new_date = NaiveDate::parse_from_str(datestring.as_str(), "%Y-%m-%d")
                                        .expect(/*datestring.as_str()*/ "Error in dartetete")
                                        .to_string();
        }

        let new_rating: u32 = get_input("Enter new rating if you wish to change it")
                                    .trim()
                                    .parse()
                                    .expect("Rating was incorrectly entered...");
        let new_note: String = get_input("Enter new note if you wish to change it")
                                    .trim()
                                    .to_owned();

        list_edit(rl, &mut name, new_name, new_date, new_rating, new_note);
    }

    fn get_date_from_input() -> String {
        let y: String = get_input("Enter new year (eg 2020, 2016, 1984)")
                            .trim()
                            .to_string();
        let m: String = get_input("Enter new month (eg 02, 07, 10)")
                            .trim()
                            .to_string();
        let d: String = get_input("Enter new day (eg 05, 19, 31)")
                            .trim()
                            .to_string();

        return format!("{0}-{1}-{2}", y, m, d);
    }

    fn update_rating(rl: &mut RatedList) {
        let mut name: String = get_input("Enter name of entry to update")
                                    .trim()
                                    .to_owned();
        let new_rating: u32 = get_input("Enter new rating")
                                    .trim()
                                    .parse()
                                    .expect("Rating was incorrectly entered...");

        list_update_rating(rl, &mut name, new_rating);
    }

    fn print_list(rl: &mut RatedList) {
        println!(">>>Printing list<<<");
        println!("{}", list_to_string(rl));
    }

    fn save_list(rl: &mut RatedList) {
        println!("Saving {}", rl.get_name());
        match list_save(rl) {
            Ok(_c) => return,
            Err(_e) => println!("!!!ERROR IN FILE HANDLING THINGY!!!"),
        }
    }

    fn export_list(rl: &mut RatedList) {
        export(rl);
    }

    fn print_wl_from_rl(rl: &mut RatedList) {
        let mut wl: WatchList = watchlist_load_or_create(rl.get_name());

        print_watchlist(&mut wl);
    }

    // Watchlist functions

    fn add_to_watchlist(wl: &mut WatchList) {
        println!(">>>Adding to watchlist<<<");
        let name: String = get_input("Enter name of entry")
                                .trim()
                                .to_owned();
        let note: String = get_input("Enter note (may leave empty)")
                                .trim()
                                .to_owned();
        watchlist_add(wl, name, note);
    }

    fn remove_from_watchlist(wl: &mut WatchList) {
        println!(">>>Removing from watchlist<<<");
        let mut name: String = get_input("What entry do you want to remove?")
                                    .trim()
                                    .to_owned();
        watchlist_remove(wl, &mut name);
        return;
    }

    fn edit_watchlist(wl: &mut WatchList) {
        let mut name: String = get_input("Enter current name of entry to edit")
                                    .trim()
                                    .to_owned();
        let new_name: String = get_input("Enter new name if you wish to change it")
                                    .trim()
                                    .to_owned();


        let mut new_date: String = String::new();
        let change_date: String = get_input("Do you wish to edit the date?, y for yes");
        if change_date.to_lowercase().chars().nth(0).unwrap() == 'y' {
            let datestring: String = get_date_from_input().trim().to_string();
            println!("{}", datestring);
            new_date = NaiveDate::parse_from_str(datestring.as_str(), "%Y-%m-%d")
                                        .expect(/*datestring.as_str()*/ "Error in dartetete")
                                        .to_string();
        }

        let new_note: String = get_input("Enter new note if you wish to change it")
                                    .trim()
                                    .to_owned();

        watchlist_edit(wl, &mut name, new_name, new_date, new_note);
    }

    fn print_watchlist(wl: &mut WatchList) {
        println!(">>>Printing watchlist<<<");
        println!("{}", watchlist_to_string(wl));
    }

    fn save_watchlist(wl: &mut WatchList) {
        println!("Saving {}", wl.get_name());
        match watchlist_save(wl) {
            Ok(_c) => return,
            Err(_e) => println!("!!!ERROR IN FILE HANDLING THINGY!!!"),
        }
    }
}