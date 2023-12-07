pub mod day3 {
    use std::fs::File;
    use std::io;
    use regex::Regex;

    pub fn calculate(lines: io::Result<io::Lines<io::BufReader<File>>>) {
        let values = get_input(lines);
        let mut to_exclude: Vec<(i32, usize, usize)> = Vec::new();
        for i in 0..values.len() {
            let numbers = &values[i].0;
            let symbols = &values[i].1;
            let mut exclude_own_line = exclude_from_line(numbers, symbols);
            if i > 0 {
                let symbols_previous_line = &values[i - 1].1;
                exclude_own_line = exclude_from_line(&exclude_own_line, symbols_previous_line);
            }
            if i < values.len() -1 {
                let symbols_next_line = &values[i + 1].1;
                exclude_own_line = exclude_from_line(&exclude_own_line, symbols_next_line);
            }
            to_exclude.extend(exclude_own_line);
        }
        println!("to_exclude: {:?}", to_exclude);

        let mut sum = 0;
        for number in values {
            for number_value in number.0 {
                sum += number_value.0;
            }
        }
        for number in to_exclude {
            sum -= number.0;
        }
        println!("sum is {:?}", sum);
    }

    fn get_input(lines: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<(Vec<(i32, usize, usize)>, Vec<usize>)> {
        let mut values: Vec<(Vec<(i32, usize, usize)>, Vec<usize>)> = Vec::new();
        match lines {
            Ok(lines) => {
                for line in lines {
                    if let Ok(ip) = line {
                        let numbers = extract_numbers(&ip);
                        println!("numbers: {:?}", numbers);
                        let symbols = extract_symbols(&ip);
                        println!("symbols: {:?}", symbols);
                        values.push((numbers, symbols));
                    }
                }
                println!("values: {:?}", values);
            },
            Err(_) => panic!("Couldn't read file"),
        };
        values
    }

    pub fn extract_numbers(text: &str) -> Vec<(i32, usize, usize)> {
        let mut numbers: Vec<(i32, usize, usize)> = Vec::new();
        let number_regex = Regex::new(r#"\d+"#).expect("why?");
        number_regex.find_iter(text).for_each(|it| {
            let number = it.as_str().parse().unwrap();
            numbers.push((number, it.start(), it.end()));
        });
        numbers
    }

    pub fn extract_symbols(text: &str) -> Vec<usize> {
        let symbol_regex = Regex::new(r#"[^.\d]"#).expect("why?");
        let symbols = symbol_regex.find_iter(text).map(|it| it.start()).collect();
        symbols
    }

    fn exclude_from_line(numbers: &Vec<(i32, usize, usize)>, symbols: &Vec<usize>) -> Vec<(i32, usize, usize)> {
        let mut excluded: Vec<(i32, usize, usize)> = Vec::new();
        for number in numbers {
            let mut exclude = true;
            for symbol_location in symbols {
                let lower_bound = match number.1.checked_sub(1) {
                    Some(value) => value,
                    None => 0,
                };
                if (symbol_location >= &lower_bound) && (symbol_location <= &(number.2)) {
                    exclude = false;
                }
            }
            if exclude {
                excluded.push(number.to_owned());
            }
        }
        excluded
    }

    #[cfg(test)]
    #[test]
    fn example_1() {
    }
    #[test]
    fn example_2() {
    }
}


