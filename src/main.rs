use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Parsing: {}", filename);

    let filepath = Path::new(filename);

    // File must exist in the current path
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let line_encoding = find_encoding(ip);
                sum += line_encoding;
                // println!("{}", line_encoding);
            }
        }
        println!("The sum is {}", sum);
    } else {
        panic!("Couldn't read file");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_encoding(line: String) -> u32 {
    let mut first_number: u32 = 0;
    let mut last_number: u32 = 0;
    for char in line.chars() {
        if let Some(number) = char.to_digit(10) {
            if first_number == 0 {
                // First number is een tiental
                first_number = number * 10;     
            }
            last_number = number;
        }
    }
    first_number + last_number
}
