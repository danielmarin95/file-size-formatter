use std::{env, fs::File};

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize{
    fn print_self(&self){
        match self {
            FileSize::Bytes(size) => println!("Bytes: {}", size),
            FileSize::Kilobytes(size) => println!("Kilobytes: {}", size),
            FileSize::Megabytes(size) => println!("Megabytes: {}", size),
            FileSize::Gigabytes(size) => println!("Gigabytes: {}", size),
        }
    }
}

fn show_sizes(size: u64, unit: &str) {
    let bytes;
    let kilobytes;
    let megabytes;
    let gigabytes;

    let converted_unit = String::from(unit);
    match &*converted_unit {
        "b" | "bytes" | "bs" => {
            bytes = FileSize::Bytes(size);
            kilobytes = FileSize::Kilobytes(size / 1024);
            megabytes = FileSize::Megabytes(size / 1024 / 1024);
            gigabytes = FileSize::Gigabytes(size / 1024 / 1024 / 1024);
        },
        "kb" | "kilobytes" | "kbs" => {
            bytes = FileSize::Bytes(size * 1024);
            kilobytes = FileSize::Kilobytes(size);
            megabytes = FileSize::Megabytes(size / 1024);
            gigabytes = FileSize::Gigabytes(size / 1024 / 1024);
        },
        "mb" | "megabytes" | "mbs" => {
            bytes = FileSize::Bytes(size * 1024 * 1024);
            kilobytes = FileSize::Kilobytes(size * 1024);
            megabytes = FileSize::Megabytes(size);
            gigabytes = FileSize::Gigabytes(size / 1024);
        },
        "gb" | "gigabyes" | "gbs" => {
            bytes = FileSize::Bytes(size * 1024 * 1024 * 1024);
            kilobytes = FileSize::Kilobytes(size * 1024 * 1024);
            megabytes = FileSize::Megabytes(size * 1024);
            gigabytes = FileSize::Gigabytes(size);
        },
        _ => panic!("Invalid unit"),
    };

    bytes.print_self();
    kilobytes.print_self();
    megabytes.print_self();
    gigabytes.print_self();

   
}


fn main() {
    // Ask user for a file size
    println!("Enter a file size (e.g. # b/kb/mb/gb): ");	
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    // Trim the input to remove any leading/trailing whitespace
    let input = input.trim();

    // Access the first part of the input and convert it to a number
    let size = input.split(' ').nth(0).unwrap().parse::<u64>().unwrap();

    // Access the second part of the input
    let unit = input.split(' ').nth(1).unwrap().to_lowercase();

    // Pass the unit as a string slice
    show_sizes(size, &unit)
}

