#![allow(non_snake_case)]
use std::env::args;
use std::process::exit;

//Usage: conv [b|o|x|d]<input> [b|o|x|d]

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: conv [b|o|x|d]<input> [b|o|x|d]");
        exit(0);
    }
    let input: u32;
    let inType = args[1].trim().get(..1).unwrap();
    match inType {
        "b" => input = u32::from_str_radix(&args[1].get(1..).unwrap(), 2).expect("Not a binary number"),
        "o" => input = u32::from_str_radix(&args[1].get(1..).unwrap(), 8).expect("Not a octal number"),
        "x" => input = u32::from_str_radix(&args[1].get(1..).unwrap(), 16).expect("Not a hexadecimal number"),
        "d" => input = u32::from_str_radix(&args[1].get(1..).unwrap(), 10).expect("Not a decimal number"),
        _ => input = u32::from_str_radix(&args[1].get(0..).unwrap(), 10).expect("Invalid number"),
    };
    let outType = args[2].trim();
    match outType {
        "b" => println!("{:b}", input),
        "o" => println!("{:o}", input),
        "x" => println!("{:X}", input),
        "d" => println!("{}", input),
        _ => println!("Usage: conv [b|o|x|d]<input> [b|o|x|d]"),
    };
}