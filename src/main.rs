fn main() {
    loop {
        let n = user_input_num("Enter a number (positive integer only)");
        println!("Fibonacci number of {} is {}", n, fibonacci(n));

        let user_end = user_input_end("Do you want to end? (y/n)");
        if user_end == "y" {
            println!("Bye!");
            break;
        }
    }


}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        return 1;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn user_input_num(prompt: &str) -> u32 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input")
    }
}

fn user_input_end(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}