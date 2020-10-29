use std::io;

fn main(){
    let mut input = String::new();
    
    println!("please input fibonacci number n:");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let n:i32 = input.trim().parse().unwrap();
    
    let finum:i32 = fibonacci(n);

    println!("the fibonacci number of n is:{}",finum);
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}