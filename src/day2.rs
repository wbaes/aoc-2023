pub mod day2 {
    use std::fs::File;
    use std::io;
    use regex::Regex;

    #[derive(Debug)]
    struct Game {
        id: i32,
        red_max: i32,
        blue_max: i32,
        green_max: i32,
    }
    impl Game {
        fn is_game_possible(&self) -> bool {
            self.blue_max <= 14 && self.red_max <= 12 && self.green_max <= 13
        }
        fn power(&self) -> i32 {
            self.blue_max * self.red_max * self.green_max
        }
    }


    pub fn calculate(lines: io::Result<io::Lines<io::BufReader<File>>>) {
        let values = get_input(lines);
        let mut sum = 0;
        let mut sum_powers = 0;
        for game in values {
            if game.is_game_possible() {
                sum += game.id;
            }
            sum_powers += game.power();
        }
        println!("sum: {}", sum);
        println!("sum powers: {}", sum_powers);
    }

    fn get_input(lines: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<Game> {
        let mut values: Vec<Game> = Vec::new();
        match lines {
            Ok(lines) => {
                for line in lines {
                    if let Ok(ip) = line {
                        let split_line = ip.split(':');
                        let mut game_id = 0; 
                        let mut red = 0;
                        let mut blue = 0;
                        let mut green = 0;
                        for part in split_line {
                            if part.contains("Game") {
                                game_id = extract_number(part);
                            } else {
                                if let Ok(red_regex) = Regex::new(
                                    r#"(\d+\sred)"#
                                    ) {
                                    red = red_regex.find_iter(part).map(|it| extract_number(it.as_str())).max().unwrap();
                                }
                                if let Ok(blue_regex) = Regex::new(
                                    r#"(\d+\sblue)"#
                                    ) {
                                    blue = blue_regex.find_iter(part).map(|it| extract_number(it.as_str())).max().unwrap();
                                }
                                if let Ok(green_regex) = Regex::new(
                                    r#"(\d+\sgreen)"#
                                    ) {
                                    green = green_regex.find_iter(part).map(|it| extract_number(it.as_str())).max().unwrap();
                                }
                            }
                        }
                        values.push(Game {id: game_id, red_max: red, blue_max: blue, green_max: green})
                    }
                }
            },
            Err(_) => panic!("Couldn't read file"),
        };
        values
    }

    fn extract_number(text: &str) -> i32 {
        let number_regex = Regex::new(r#"\d+"#).expect("No number!");
        let number: i32 = number_regex.find(text).map(|t| t.as_str().parse().unwrap()).unwrap();
        number
    }
}

