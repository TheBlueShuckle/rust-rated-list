mod ui;
mod list_handler;

use ui::ui::run;

use crate::list_handler::RatedList;

fn main() {
    let mut lists: Vec<RatedList> = Vec::new();

    run(&mut lists);
}
