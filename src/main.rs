mod modules;

use modules::hash;
use modules::io::fs::get_metadata;
use modules::Logging::ErrorLog;

fn main() {
    let mut er = ErrorLog::new();
    let path = std::path::Path::new("./Cargo.loc");
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
