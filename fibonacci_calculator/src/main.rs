use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter a positive number to calculate the nth Fibonacci number:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line.");
    let number: u128 = user_input.trim().parse().expect("Please enter a valid positive number.");
    
    println!("Calculating Fibonacci for {}...", user_input);
    let answer = fibonacci_memo(number);
    println!("The Fibonacci number is: {}", answer);
}

fn fibonacci_memo(number: u128) -> u128 {
    let mut memo: HashMap<u128, u128> = HashMap::new();
    fibonacci_recursive(number, &mut memo)
}

fn fibonacci_recursive(number: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if number == 0 {
        return 0;
    } else if number == 1{
        return 1;
    }
    
    else if let Some(&result) = memo.get(&number) {
        return result;
    }

    else {
        let result = fibonacci_recursive(number - 1, memo) + fibonacci_recursive(number -2, memo);
        memo.insert(number,result);
        return result
    }
}
