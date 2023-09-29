use std::borrow::Cow;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, error::Error};

pub struct ErrorLog {
    map: HashMap<String, Box<dyn Error>>,
}

impl ErrorLog {
    pub fn set(&mut self, k: &Cow<'_, str>, v: Box<dyn Error>) {
        self.map.insert(k.to_string(), v);
    }

    pub fn new() -> ErrorLog {
        let error_map: HashMap<String, Box<dyn Error>> = std::collections::HashMap::new();

        let logger = ErrorLog { map: error_map };
        logger
    }

    pub fn write_error_file(&self) {
        let mut output_file = fs::File::create("./error_log").expect("Cannot Create Error Log");

        for (filename, as_secs) in &self.map {
            writeln!(output_file, "{}: {}", filename, as_secs)
                .expect("Failed to Write to Error Log");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;

    #[derive(Debug)]
    struct MyError(String);

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for MyError {}

    #[test]
    fn error_logger_new() -> Result<(), &'static str> {
        let er = ErrorLog::new();

        match er {
            ErrorLog => Ok(()),
            _ => Err("MEEP"),
        }
    }

    #[test]
    fn error_log_set() {
        let mut er = ErrorLog::new();
        let kt = Cow::Borrowed("YOLO");

        er.set(&kt, Box::new(MyError("SWAG".to_string())));

        let x = er.map.get("YOLO");
        assert!(x.is_some())
    }

    #[test]
    fn error_write() {
        let mut er = ErrorLog::new();

        let kt = Cow::Borrowed("YOLO");
        let kf = Cow::Borrowed("GWALF");

        er.set(&kt, Box::new(MyError("SWAG".to_string())));
        er.set(&kf, Box::new(MyError("MEEP".to_string())));

        er.write_error_file()
    }
}
