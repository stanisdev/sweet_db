extern crate file_worker;

use std::env;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::io::SeekFrom;
use std::io::Seek;
use std::io::{BufReader};
use std::io::prelude::*;

pub struct FileWorker {
    descriptor: File
}

impl FileWorker {

    // Create instance
    pub fn new() -> FileWorker {
        let mut file_path = env::current_dir().unwrap();
        file_path.push("src/data/data.txt");

        match OpenOptions::new().read(true).write(true).open(&file_path) {
            Err(why) => {
                panic!("File cannot be opened: {}", why);
            },
            Ok(file) => {
                return FileWorker {
                    descriptor: file
                };
            },
        };
    }

    // * Write string
    pub fn write(&mut self) {
        self.descriptor.seek(SeekFrom::End(0));

        let mut buf = BufWriter::new(&self.descriptor);
        let data = "Some data23!\n";
        match buf.write(data.as_bytes()) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(file) => println!("Done, {:?}", file),
        }
    }

    // Read from file
    pub fn read(&mut self) {
        let br = BufReader::new(&self.descriptor);
        for line in br.lines() {
            match line {
                Err(_) => {},
                Ok(data) => println!("{}", data),
            }
        }
    }
}
