pub mod index {
    use std::fs::OpenOptions; 
    use std::io::Write;
    use std::path::Path;

    pub fn write_log_to_file(log: &str, file_name: &str) {
        let mut path = String::from("logs/");
        path.push_str(file_name);
        let path_from_string = Path::new(&path);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path_from_string)
            .expect("Something");

        file.write_all(log.as_bytes());
    }

}



