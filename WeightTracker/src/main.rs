#![feature(io)]
#![feature(collections)]

use std::old_io;

fn main() {
    loop {
        let user_input = old_io::stdin().read_line().ok().expect("Failed to read input");
        let exit = process_input(&user_input);
        if exit {return;}
    }
}

fn process_input(input: &str) -> bool {
    match input {
        "i" | "I" => input_weights(),
        _ => return false
    };
    false
}

fn input_weights() {
    let mut date = "2015.11.02";
    loop {
        print!("{}\t", date);
        let user_input = old_io::stdin().read_line().ok().expect("Failed to readinput");
        let weight: Option<f32> = validate_weight_input(&user_input);
        match weight {
            Some(weight) => save_weight_to_file(date, weight),
            None => {
                match validate_date_input(&user_input) {
                    Option::Some(new_date) => date = new_date,
                    Option::None => continue
                }
            }
        }
    }
}

fn validate_weight_input(weight_input: &str) -> Option<f32> {
    match weight_input.trim().parse::<f32>() {
        Some(weight) => {
            if weight < 0.0 { Option::None }
            else { Option::Some(weight) }
        },
        None => Option::None
    }
}

fn validate_date_input(date_input: &str) -> Option<&str> {
    let parts: Vec<&str> = date_input.trim().split('.').collect();
    if parts.len() < 2 || parts.len() > 3 { return Option::None; }
    
    let month = match get_month_input(parts[parts.len() - 2]) {
        Option::Some(month) => month,
        Option::None => return Option::None
    };
    if !correct_day_input(parts[parts.len() - 1], month) { return Option::None; }
    if parts.len() == 3 {
        if !correct_year_input(parts[0]) { return Option::None; }
    }
    
    Option::Some(date_input)
}

fn correct_day_input(day_input: &str, month: u8) -> bool {
    let day = match day_input.parse::<u8>() {
        Option::Some(day) => day,
        Option::None => return false
    };
    
    if day < 1 { return false; }
    
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => day <= 29,
        _ => false
    }
}

fn get_month_input(month_input: &str) -> Option<u8> {
    match month_input.parse::<u8>() {
        Option::Some(month) => {
            if month > 0 && month < 13 { return Option::Some(month); }
            else { return Option::None; }
        },
        Option::None => return Option::None
    }
}

fn correct_year_input(year_input: &str) -> bool {
    match year_input.parse::<i32>() {
        Option::Some(_) => true,
        Option::None => false
    }
}

fn save_weight_to_file(date: &str, weight: f32) {

}