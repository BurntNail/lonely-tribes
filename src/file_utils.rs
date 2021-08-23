use std::path::Path;
use std::fs::read_dir;

///Gets file names inside a directory
pub fn list_file_names_in_dir<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut list = Vec::new();
    if let Ok(read) = read_dir(path) {
        read.for_each(|el| {
            if let Ok(el) = el {
                let current_file = format!("{:?}", el.file_name());
                list.push(current_file);
            }
        });

        list.sort();
        list.reverse();
    }

    list
}