pub mod day1 {
    use std::fs::File;
    use std::io;
    use std::collections::HashMap;

    pub fn calculate(lines: io::Result<io::Lines<io::BufReader<File>>>) {
        match lines {
            Ok(lines) => {
                let mut sum: u32 = 0;
                for line in lines {
                    if let Ok(ip) = line {
                        let line_encoding = find_encoding(ip);
                        sum += line_encoding;
                        // println!("{}", line_encoding);
                    }
                }
                println!("The sum is {}", sum);
            },
            Err(_) => panic!("Couldn't read file"),
        };
    }


    fn find_encoding(line: String) -> u32 {
        let mut first_number: u32 = 0;
        let mut last_number: u32 = 0;

        let transformed_line = insert_numbers_before_equivalent_text(line);

        for char in transformed_line.chars() {
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

    fn insert_numbers_before_equivalent_text(mut line: String) -> String {

        // Mapping from text to number
        let mut string_to_number = HashMap::new();
        string_to_number.insert(String::from("one"), String::from("1"));
        string_to_number.insert(String::from("two"), String::from("2"));
        string_to_number.insert(String::from("three"), String::from("3"));
        string_to_number.insert(String::from("four"), String::from("4"));
        string_to_number.insert(String::from("five"), String::from("5"));
        string_to_number.insert(String::from("six"), String::from("6"));
        string_to_number.insert(String::from("seven"), String::from("7"));
        string_to_number.insert(String::from("eight"), String::from("8"));
        string_to_number.insert(String::from("nine"), String::from("9"));

        // Find out where to insert the numbers by looking for ooccurences of the numbers as strings
        // and recording the position
        // Note: replacing doesn't work because one letter might belong to 2 numbers (eg. eightwo ->
        // eight and two)
        let mut to_insert: Vec<(usize, &String)> = Vec::new();
        for (text, number) in &string_to_number {
            for (_, value) in line.match_indices(text).enumerate() {
                to_insert.push((value.0, number))
            }
        }
        // Make sure the vector is sorted by insertion index
        to_insert.sort_by_key(|k| k.0);
        let mut inserted = 0;
        // Loop through the vector in a fixed order
        for index in 0..to_insert.len() {
            // Add the amount already inserted to the index, since that index was calculated on the
            // original line, this one might have insertions before this one
            line.insert_str(to_insert[index].0 + inserted, to_insert[index].1);
            inserted += 1;
        }
        line
    }
}
