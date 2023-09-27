mod modules;

use modules::Logging::Log::ErrorLog;

use std::fs;

use std::{error::Error, time::SystemTime};

fn main() {
    let mut er = ErrorLog::new();
    let path = std::path::Path::new("./Cargo.lock");
    match get_metadata(&path) {
        Ok(x) => {
            x;
        }
        Err(e) => {
            er.set(&path.to_string_lossy(), e);
        }
    }

    er.write_error_file();
}

fn get_metadata(path: &std::path::Path) -> Result<SystemTime, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(&path);

    let k: Result<SystemTime, Box<dyn Error>> = match metadata {
        Ok(x) => match x.modified() {
            Ok(y) => Ok(y),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    };
    k
}
