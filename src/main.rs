use std::io;

fn main() {
    let mut input = String::new();
    println!("Where to finish: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let input: i32 = input.trim().parse().expect("Failed to parse input");

    calculate_fibonacci(input);
}

fn calculate_fibonacci(n: i32) {
    let mut num1 = 0;
    let mut num2 = 1;
    let mut next_num = 0;

    while next_num <= n {
        println!("num1: {}, num2: {}, next_num: {}", num1, num2, next_num);
        
        num1 = num2;
        num2 = next_num;
        next_num = num1 + num2; 
    }

}