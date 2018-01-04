use error::*;
use package::DownloadPackage;
use std::io::Read;
use md5::{Md5, Digest};
use std::fs::File;
use std::io::Write;

/// Trait to write a stream of data to a file.
pub trait FileWriter : Read {
    /// Function to write a stream of data, to a file
    /// based on the std::io::Read trait. This functions
    /// returns as result the hash of the written file.
    fn write_to_file<S: Into<String>>(&mut self, file: S) -> Result<String> {
        // define the buffer
        let mut buffer = [0u8; 32];

        // define the hasher
        let mut hasher = Md5::new();

        // Create the output file
        let mut file = File::create(file.into())?;

        // print out the values
        loop {
            // read the data from the stream
            let len = self.read(&mut buffer)?;

            // break if no data is available anymore
            if len == 0 {
                break;
            }

            // sent the data to the file and hasher
            hasher.input(&buffer[0..len]);
            file.write(&buffer[0..len]);
        }

        // return the hash as a string
        Ok(format!("{:x}", hasher.result()))
    }
}

// implement the Download Reader for the reqwest response
impl FileWriter for ::reqwest::Response{}