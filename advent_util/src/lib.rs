use std::fs::File;
use std::io::Read;

pub fn read_input(day: u8) -> Result<String, String> {
    let mut input_file = File::open(format!("day{}/input.txt", day))
        .map_err(|_| format!("Download file to day{} with 'download.http'", day))?;
    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .map_err(|e| format!("Cannot read from file: {:?}", e))?;
    Ok(input)
}
