pub mod list_exporter {
    use std::{fs::File, io::Write};
    use crate::list_handler::{RatedList};

    pub fn export(rl: &mut RatedList) {
        let file_name: String = format!("out/{}.md", rl.get_name());
        let mut file: File = File::create(file_name).expect("Failed to create MD file...");

        let title: String = format!("# {}", rl.get_name());
        let _ = file.write(title.as_bytes());

        for name in rl.entry_names() {
            let _ = file.write(b"\n");
            let _ = file.write(rl.entry_to_string(name).as_bytes());
        }
    }
}