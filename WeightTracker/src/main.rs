#![feature(io)]
#![feature(collections)]

use std::old_io;

struct WeightAtDate {
    date: String,
    weight: f32
}

enum Action {
    InputWeights,
    Exit,
    Unknown(String)
}

fn main() {
    let mut weights = Vec::<WeightAtDate>::new();
    
    loop {
        let user_input = old_io::stdin().read_line().ok().expect("Failed to read input");
        match process_input(user_input.trim()) {
            Action::InputWeights => input_weights(&mut weights),
            Action::Exit => return,
            Action::Unknown(s) => println!("Invalid input: {}", s)
        }
    }
}

fn process_input(input: &str) -> Action {
    match input {
        "i" | "I" => Action::InputWeights,
        "x" | "X" => Action::Exit,
        _ => Action::Unknown(input.to_string())
    }
}

fn input_weights(weights: &mut Vec<WeightAtDate>) {
    let mut date = "2015.02.11".to_string();
    loop {
        print!("{}\t", date);
        let user_input = old_io::stdin().read_line().ok().expect("Failed to read input");
        let weight = validate_weight_input(user_input.trim().parse::<f32>());
        match weight {
            Some(weight) => weights.push(WeightAtDate {date: date.clone(), weight: weight}),
            None => {
                if valid_date_input(&user_input) { date = user_input.trim().to_string(); }
            }
        }
        println!("\nWeights inserted so far:\n");
        for element in weights.iter() { println!("{}: {}", element.date, element.weight); }
        println!("");
    }
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

fn valid_date_input(date_input: &str) -> bool {
    let parts: Vec<&str> = date_input.trim().split('.').collect();
    if parts.len() != 3 { return false; }
    
    if !correct_year_input(parts[0]) { return false; }
    let month = match validate_month_input(parts[1].parse::<u8>()) {
        Option::Some(month) => month,
        Option::None => return false
    };
    if !correct_day_input(parts[2], month) { return false; }
    
    true
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

fn validate_month_input(month_input: Option<u8>) -> Option<u8> {
    match month_input {
        Some(month) => {
            if month > 0 && month < 13 { return Some(month); }
            else { return None; }
        },
        None => return None
    }
}

fn correct_year_input(year_input: &str) -> bool {
    match year_input.parse::<i32>() {
        Option::Some(_) => true,
        Option::None => false
    }
}