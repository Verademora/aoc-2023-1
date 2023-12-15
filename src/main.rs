use std::fs::File; 
use std::io::prelude::*;
use std::error::Error; 

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    
    let v: Vec<&str> = buffer.trim().split('\n').collect();

    Ok(())
}
