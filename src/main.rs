use std::fs::File; 
use std::io::prelude::*;
use std::error::Error; 

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    
    let v: Vec<&str> = buffer.trim().split('\n').collect();
    
    let mut result = 0;
    
    for line in v {
        let i = line.find(|c: char| c.is_ascii_digit()).unwrap(); 
        let first: i32 = line.get(i..i+1).unwrap().parse().unwrap();
        let i = line.rfind(|c: char| c.is_ascii_digit()).unwrap(); 
        let last: i32 = line.get(i..i+1).unwrap().parse().unwrap();
        result += first * 10 + last;
    }
    
    println!("Final result: {:?}", result);

    Ok(())
}
