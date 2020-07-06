use std::io;
use mylib;

fn main() {
    println!("please input the year:");
    let mut year = String::new();

    io::stdin().read_line(&mut year).expect("Error: failed to get year!");
    let year_int:i32 = year.trim().parse().unwrap();

    let tag:bool = mylib::leap_year_judgement::is_leap(year_int);
    if tag == true {
        println!("the {} is a leap year!", year);
    } else {
        println!("the {} is not a leap year!", year);
    }
}
