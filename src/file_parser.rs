use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct FileParser{
    pub m_url: String,
    m_raw_data: String
}

impl FileParser{
    pub fn new(url:String) -> FileParser{
        FileParser{
            m_url: url,
            m_raw_data: String::new()
        }
    }

    pub fn get_raw_data(&self) -> String{
        return self.m_raw_data.clone();
    }
    
    pub fn get_raw_data_from_file(&mut self) {
        let path = Path::new(&self.m_url);
        let display = path.display();
    
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        match file.read_to_string(&mut self.m_raw_data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("{} parsed into a string", display),
        }
    }
}

