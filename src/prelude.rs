pub use serde::{Serialize, Deserialize};
pub use crate::structs::*;
use std::path::PathBuf;
use std::io::{Read, Write};

const DATA_FILE_PATH: &str = "eBookmark_data.json";

pub type EBookmarkResult<T> = Result<T, failure::Error>;

pub fn read_data() -> EBookmarkResult<EBookmarkData> {
    let mut data_file = {
        let data_file_path = PathBuf::from(DATA_FILE_PATH);
        std::fs::File::open(&data_file_path)?
    };

    let data_string = {
        let mut string = String::new();
        data_file.read_to_string(&mut string)?;
        string
    };

    match serde_json::from_str::<EBookmarkData>(&data_string) {
        Ok(data) => Ok(data),
        Err(e) => Err(format_err!("{}",&e)),
    }
}

pub fn write_data(data: &EBookmarkData) -> EBookmarkResult<()>{
    let mut data_file = {
        let data_file_path = PathBuf::from(DATA_FILE_PATH);
        std::fs::File::create(&data_file_path)?
    };

    let string = match serde_json::to_string(&data) {
        Ok(s) => s,
        Err(e) => return Err(format_err!("{}",&e)),
    };

    data_file.write_all(string.as_bytes())?;
    data_file.flush()?;

    Ok(())

}