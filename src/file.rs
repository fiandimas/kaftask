pub mod file {
    use std::fs;
    use std::io::{Result, Write};

    const FILE: &str = "tasks.txt";

    pub fn read() -> Result<String>  {
        match fs::read_to_string(FILE) {
            Ok(contents) => Ok(contents),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => create(),
            Err(e) => Err(e),
        }
    }

    pub fn append(is_first: bool, text: String) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(FILE)?;

        if is_first {
            write!(file, "{text}")?;
        } else {
            write!(file, "|{text}")?;
        }

        Ok(())
    }

    pub fn overwrite(text: String) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE)?;

        write!(file, "{text}")?;

        Ok(())
    }

    fn create() -> Result<String> {
        fs::write(FILE, "")?;

        Ok(String::from(""))
    }
}