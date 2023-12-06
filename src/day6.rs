pub mod day6 {
    use std::fs::File;
    use std::io;

    pub fn calculate(lines: io::Result<io::Lines<io::BufReader<File>>>) {
        let values = get_input(lines);
        let amount_of_races = values.len()/2;
        let times = &values[..amount_of_races];
        let distances = &values[amount_of_races..];
        let mut product = 1;
        for race in 0..amount_of_races {
            product *= find_better_races(times[race], distances[race]);
        }
        println!("The product is {}", product);
    }

    fn get_input(lines: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<usize> {
        let mut values: Vec<usize> = Vec::new();
        match lines {
            Ok(lines) => {
                for line in lines {
                    if let Ok(ip) = line {
                        let split_line = ip.split(':');
                        for part in split_line {
                            let value: Result<usize, _> = part.to_string().replace(" ", "").parse();
                            if let Ok(value) = value {
                                values.push(value);
                            }
                        }
                    }
                }
                println!("values: {:?}", values);
            },
            Err(_) => panic!("Couldn't read file"),
        };
        values
    }

    fn find_better_races(time: usize, best_distance: usize) -> i32 {
        let mut amount_better_races = 0;
        for time_button_held_down in 0..time {
            let time_left = time - time_button_held_down;
            if time_left * time_button_held_down > best_distance {
                amount_better_races += 1;
            } 
        }
        amount_better_races
    }

    #[cfg(test)]
    #[test]
    fn example_1() {
        assert_eq!(find_better_races(7, 9), 4);
        assert_eq!(find_better_races(15, 40), 8);
        assert_eq!(find_better_races(30, 200), 9);
    }
    #[test]
    fn example_2() {
        assert_eq!(find_better_races(71530, 940200), 71503);
    }
}

