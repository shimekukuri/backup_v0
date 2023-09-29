use std::{
    collections::VecDeque,
    error::Error,
    fs::{self, DirEntry, File},
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    time::SystemTime,
};

use rayon::prelude::*;

pub fn get_metadata(path: &std::path::Path) -> Result<SystemTime, Box<dyn std::error::Error>> {
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

pub fn scan_tree(path: &std::path::Path, handle_file: fn(entry: DirEntry)) {
    let mut queue: VecDeque<DirEntry> = VecDeque::new();
    read_into_queue(std::path::Path::new("./"), &mut queue);

    while queue.len() > 0 {
        let current_val = queue.pop_front().unwrap();
        if current_val.file_type().unwrap().is_dir() {
            read_into_queue(&current_val.path(), &mut queue)
        } else {
            handle_file(current_val);
        }
    }
}

fn read_into_queue(path: &std::path::Path, queue: &mut VecDeque<DirEntry>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            match entry {
                Ok(x) => {
                    &queue.push_back(x);
                }
                Err(e) => {
                    //do error stuff
                    //prob log error
                }
            }
        }
    }
}

fn send_file_to_server(file_path: &str, server_address: &str) -> Result<(), io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 4096];

    // Connect to the remote server
    let mut stream = TcpStream::connect(server_address)?;

    // Read the file and send its content to the server
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(n) => {
                if let Err(err) = stream.write_all(&buffer[0..n]) {
                    eprintln!("Error sending data to server: {:?}", err);
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn parrallel_tcp() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = "127.0.0.1:8080"; // Replace with the actual server address

    // List of file paths to send
    let file_paths = vec!["file1.txt", "file2.txt", "file3.txt"];

    // Use Rayon to send files concurrently
    file_paths.par_iter().for_each(|file_path| {
        if let Err(err) = send_file_to_server(file_path, server_address) {
            eprintln!("Error: {:?}", err);
        } else {
            println!("File {} sent to server successfully.", file_path);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use rayon::Scope;

    use super::*;
    use std::{fs::OpenOptions, io::Write};

    #[test]
    pub fn scan_tree_test() {
        scan_tree(std::path::Path::new("./"), |x| println!("{:?}", x.path()))
    }

    #[test]
    pub fn scan_tree_write() {
        let x = fs::remove_file(std::path::Path::new("./tree.txt"));
        match x {
            Ok(()) => (),
            Err(e) => (),
        }

        scan_tree(std::path::Path::new("./"), move |x| {
            // Ignore the error if the file doesn't exist or there was another issue.

            let mut file_handler = OpenOptions::new()
                .create(true)
                .append(true)
                .open("tree.txt")
                .unwrap();
            writeln!(file_handler, "{}", x.path().to_string_lossy()).unwrap()
        })
    }

    #[test]
    pub fn rayon_test() {}
}
