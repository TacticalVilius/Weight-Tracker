#![feature(io)]
#![feature(path)]

use std::old_io;
use std::collections::HashMap;
use std::old_io::{File, Open, ReadWrite};

enum Action {
    InputWeights,
    Exit,
    Unknown(String)
}

fn main() {
    let path = Path::new("data.dat");
    
    let mut file = match File::open_mode(&path, old_io::Open, old_io::ReadWrite) {
        Ok(f) => f,
        Err(why) => panic!("Could not open data file {}: {}", path.display(), why.desc)
    };
    
    let mut weights = HashMap::new();
    populate_weights_map(&mut weights, &mut file);
    
    loop {
        let user_input = old_io::stdin().read_line().ok().expect("Failed to read input");
        match process_input(user_input.trim()) {
            Action::InputWeights => input_weights(&mut weights),
            Action::Exit => {
                file = match File::open_mode(&path, old_io::Truncate, old_io::Write) {
                    Ok(f) => f,
                    Err(why) => panic!("Could not open date file {}: {}", path.display(), why.desc)
                };
                save_weights_to_file(&weights, &mut file);
                return;
            }
            Action::Unknown(s) => println!("Invalid input: {}", s)
        }
    }
}

fn save_weights_to_file(weights: &HashMap<String, f32>, file: &mut File) {
    let write_str = weights.iter().fold("".to_string(), |s, (date, weight)| s + date + ": " + &weight.to_string() + "\n");
    match file.write_str(&write_str) {
        Err(why) => panic!("Could not write to {}: {}", file.path().display(), why.desc),
        Ok(_) => return
    }
}

fn populate_weights_map(weights_map: &mut HashMap<String, f32>, file: &mut File) {
    let file_contents = file.read_to_string().ok().expect("Could not read data from file");
    for line in file_contents.lines() {
        let line_parts: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
        if line_parts.len() != 2 {
            println!("Corrupted data in file {}: {}", file.path().display(), line);
            continue;
        }
        let date =
            if valid_date(line_parts[0]) { line_parts[0].to_string() }
            else {
                println!("Corrupted data in file {}: {}", file.path().display(), line);
                continue;
            };
        let weight = match line_parts[1].parse::<f32>().ok() {
            Option::Some(f) => f,
            Option::None => {
                println!("Corrupted data in file {}: {}", file.path().display(), line);
                continue;
            }
        };
        weights_map.insert(date.clone(), weight);
    }
}

fn process_input(input: &str) -> Action {
    match input {
        "i" | "I" => Action::InputWeights,
        "x" | "X" => Action::Exit,
        _ => Action::Unknown(input.to_string())
    }
}

fn input_weights(weights: &mut HashMap<String, f32>) {
    let mut date = "2015.02.11".to_string();
    loop {
        print_weights(&date, weights, 4);
        print!("\n{}:\t", date);
        let user_input = old_io::stdin().read_line().ok().expect("Failed to read input");
        match process_input(user_input.trim()) {
            Action::Exit => return,
            Action::Unknown(_) => {},
            _ => continue
        }
        let weight = validate_weight_input(user_input.trim().parse::<f32>().ok());
        match weight {
            Some(weight) => {
                weights.insert(date.clone(), weight);
                let (year, month, day) = date_from_str(&date);
                let (new_year, new_month, new_day) = add_days_to_date(year, month, day, 1);
                date = str_from_date(new_year, new_month, new_day);
            },
            None => {
                if valid_date(&user_input) { date = user_input.trim().to_string(); }
            }
        }
        println!("\nWeights inserted so far:\n");
        for (key, value) in weights.iter() {
            println!("{}: {}", key, value);
        }
        println!("");
    }
}

fn print_weights(date: &str, weights: &HashMap<String, f32>, pad_num: u32) {
    let (year, month, day) = date_from_str(date);
    let (mut year_, mut month_, mut day_) = subtract_days_from_date(year, month, day, pad_num);
    
    for _ in 0..(pad_num * 2 + 1) {
        let date_str = str_from_date(year_, month_, day_);
        let weight = match weights.get(&date_str) {
            Some(weight) => weight.to_string(),
            None => "".to_string()
        };
        println!("{}:\t{}", date_str, weight);
        let (new_year, new_month, new_day) = add_days_to_date(year_, month_, day_, 1);
        year_ = new_year;
        month_ = new_month;
        day_ = new_day;
    }
}

fn add_days_to_date(mut year: i32, mut month: u8, mut day: u8, num_days: u32) -> (i32, u8, u8) {
    for _ in 0..num_days {
        if valid_day_in_month(day + 1, month) { day = day + 1; }
        else if valid_month(month + 1) {
            month = month + 1;
            day = 1;
        }
        else {
            year = year + 1;
            month = 1;
            day = 1;
        }
    }
    (year, month, day)
}

fn subtract_days_from_date(mut year: i32, mut month: u8, mut day: u8, num_days: u32) -> (i32, u8, u8) {
    for _ in 0..num_days {
        if valid_day_in_month(day -1, month) { day = day - 1; }
        else if valid_month(month - 1) {
            month = month - 1;
            day = get_last_day_in_month(month);
        }
        else {
            year = year - 1;
            month = 12;
            day = 31;
        }
    }
    (year, month, day)
}

fn valid_day_in_month(day: u8, month: u8) -> bool {
    day > 0 && day <= get_last_day_in_month(month)
}

fn get_last_day_in_month(month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29,
        _ => 0
    }
}

fn valid_month(month: u8) -> bool {
    return month > 0 && month < 13
}

fn date_from_str(date: &str) -> (i32, u8, u8) {
    let parts: Vec<&str> = date.split('.').collect();
    (parts[0].parse::<i32>().ok().expect(""), parts[1].parse::<u8>().ok().expect(""), parts[2].parse::<u8>().ok().expect(""))
}

fn str_from_date(year: i32, month: u8, day: u8) -> String {
    let year_str = year.to_string();
    let mut month_str = month.to_string();
    if month < 10 { month_str = format!("{}{}", "0", month_str); }
    let mut day_str = day.to_string();
    if day < 10 { day_str = format!("{}{}", "0", day_str); }
    
    format!("{}{}{}{}{}", year_str, ".", month_str, ".", day_str)
}

fn validate_weight_input(input: Option<f32>) -> Option<f32> {
    match input {
        Some(weight) => {
            if weight < 0.0 { Option::None }
            else { Option::Some(weight) }
        },
        None => Option::None
    }
}

fn valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.trim().split('.').collect();
    if parts.len() != 3 { return false; }
    
    if !correct_year_input(parts[0]) { return false; }
    let month = match validate_month_input(parts[1].parse::<u8>().ok()) {
        Option::Some(month) => month,
        Option::None => return false
    };
    if !correct_day_input(parts[2], month) { return false; }
    
    true
}

fn correct_day_input(day_input: &str, month: u8) -> bool {
    let day = match day_input.parse::<u8>().ok() {
        Option::Some(day) => day,
        Option::None => return false
    };
    
    if day < 1 { return false; }
    
    return valid_day_in_month(day, month);
}

fn validate_month_input(month_input: Option<u8>) -> Option<u8> {
    match month_input {
        Some(month) => {
            if valid_month(month) { return Some(month); }
            else { return None; }
        },
        None => return None
    }
}

fn correct_year_input(year_input: &str) -> bool {
    match year_input.parse::<i32>().ok() {
        Option::Some(_) => true,
        Option::None => false
    }
}