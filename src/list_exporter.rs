pub mod list_exporter {
    use std::{
        fs::File, 
        io::Write
    };
    use crate::{list_handler::RatedList, watchlist_handler::watchlist_handler::watchlist_load_or_create};

    pub fn export(rl: &mut RatedList) {
        let file_name: String = format!("out/{}.md", rl.get_name());
        let mut file: File = File::create(file_name).expect("Failed to create MD file...");

        let title: String = format!("# {}", rl.get_name());
        _ = file.write(title.as_bytes());

        for name in rl.entry_names() {
            let _ = file.write(b"\n");
            let _ = file.write(rl.entry_to_string(name).as_bytes());
        }

        let wl = watchlist_load_or_create(rl.get_name());

        _ = file.write(b"# Watchlist");

        for name in wl.entry_names() {
            _ = file.write(b"\n");
            _ = file.write(wl.entry_to_string(name).as_bytes())
        }
    }
}