pub mod list_exporter {
    use std::{fs::File, io::Write};
    use crate::list_handler::{RatedList, list_handler::list_entry_to_string};

    pub fn export(rl: &mut RatedList) {
        let file_name: String = format!("out/{}.md", rl.get_name());
        let mut file: File = File::create(file_name).expect("Failed to create MD file...");

        let title: String = format!("# {}", rl.get_name());
        let _ = file.write(title.as_bytes());

        let names = rl.get_names();
        for name in names {
            file.write(b"\n\n");
            file.write(list_entry_to_string(rl, name).as_bytes());
        }
    }
}