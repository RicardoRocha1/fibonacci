use std::io;

fn main() {
    println!("Please enter a positive number to calculate the nth Fibonacci number:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line.");
    let number: u128 = user_input.trim().parse().expect("Please enter a valid positive number.");
    
    println!("Calculating Fibonacci for {}...", user_input);
    let answer = fibonacci(number);
    println!("The Fibonacci number is: {}", answer);
}

fn fibonacci(number: u128) -> u128 {
    if number == 0 {
        return 0;
    } else if number == 1{
        return 1;
    } else {
        return (fibonacci(number - 1) + fibonacci(number -2));
    }
}
