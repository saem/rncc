use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Error as IoError;
use std::io::BufReader;
use std::io::BufRead;

pub fn parse(path: &Path) {
    let line_reader = match create_line_reader(path) {
        Err(why) => panic!("Couldn't open file: {}, because: {}",
                           path.display(),
                           why.description()),
        Ok(file) => file,
    };

    for read in line_reader.lines() {
        match read {
            Err(why) => panic!("Couldn't read line because: {}", why.description()),
            Ok(line) => println!("{}", line),
        }
    }
}

fn create_line_reader(path: &Path) -> Result<BufReader<File>, IoError> {
    return File::open(path)
        .map(|open_file| BufReader::new(open_file));
}