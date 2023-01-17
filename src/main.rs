#![allow(non_camel_case_types)]

use std::io::Write;
#[derive(Copy,Clone)]
enum ResistorCode{
    black  = 0,
    brown  = 1,
    red    = 2,
    orange = 3,
    yellow = 4,
    green  = 5,
    blue   = 6,
    violet = 7,
    grey   = 8,
    white  = 9
}

struct Resistor(ResistorCode,ResistorCode,ResistorCode);

impl Resistor{
    fn value(&self) -> u64{
        let &Self(a,b,c) = self;
        ((a as u64 * 10) + (b as u64)) * (10u64.pow(c as u32))     
    }
}

impl ResistorCode{
    fn new(color: &str) -> Option<Self>{
        match color{
             "black" => Some(ResistorCode::black),
             "brown" => Some(ResistorCode::brown),
             "red" => Some(ResistorCode::red),
             "orange" => Some(ResistorCode::orange),
             "yellow" => Some(ResistorCode::yellow),
             "green" => Some(ResistorCode::green),
             "blue" => Some(ResistorCode::blue),
             "violet" => Some(ResistorCode::violet),
             "grey" => Some(ResistorCode::grey),
             "white" => Some(ResistorCode::white),
            _ => None,
        }
    }
}

fn display_in_k(n: u64) -> String {
    let x = (n as f64 / 1000.0) * 10.0;
    let x = x.round() as i64;
    format!("{}.{}K", x / 10, x % 10)
}
fn main() {
    println!("######################################\n");
    println!("Resistor Color Code Calculator for 3 band resistor\n");
    println!("######################################\n");    
    print!("Enter the color codes : ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
    .expect("Input Read failed");
    let mut iter = input.split_whitespace();
    let color_code_1 = ResistorCode::new(iter.next().expect("1st color code missing")).expect("invalid color code");
    let color_code_2 = ResistorCode::new(iter.next().expect("2nd color code missing")).expect("invalid color code");
    let color_code_3 = ResistorCode::new(iter.next().expect("3rd color code missing")).expect("invalid color code");

    let r1 = Resistor(color_code_1,color_code_2,color_code_3);
    println!("\nresistor value is {} ohms",display_in_k(r1.value()));
}
