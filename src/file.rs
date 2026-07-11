
pub mod file {
    use std::{fs};
    use std::io::Write;

    const FILE: &str = "tasks.txt";

    pub fn read() -> String  {
        match fs::read_to_string(FILE) {
            Ok(contents) => contents,
            Err(_e) => create(),
        }
    }

    pub fn append(is_first: bool, text: String) -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true) // bikin file baru kalo belum ada
        .open(FILE)?;

        if is_first {
            write!(file, "{text}");
        } else {
            write!(file, "|{text}");
        }

        Ok(())
    } 

    fn create() -> String {
        if let Err(_e) = fs::write(FILE, "") {

        }

        return String::from("")
    }
}