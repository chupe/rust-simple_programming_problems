#![allow(unused)]
use std::io;
use std::io::BufRead;

fn main() {
    let mut stdin = String::new();
    io::stdin().read_line(&mut stdin);
    let number = stdin.trim().parse::<i32>().unwrap();
    let mut first_column = true;
    let mut first_row = true;

    for i in 1..number + 1 {
        for j in 1..number + 1 {
            let digits = number.to_string().len();
            let mut spaces = String::new();
            for k in 0..(5 - digits) {
                spaces.push_str(" ");
            }
            if first_row {
                print!(" ");
                for l in 1..number + 1 {
                    print!("{}{}", spaces, l);
                }
                print!("\n");
                first_row = false;
            }
            if first_column {
                print!("{}", i);
                first_column = false;
            }
            let product = i * j;
            let digits = product.to_string().len();
            spaces = String::new();
            for k in 0..(5 - digits) {
                spaces.push_str(" ");
            }
            print!("{}{}", spaces, i * j);
        }
        first_column = true;
        print!("\n");
    }
    println!("{}", number);
}
