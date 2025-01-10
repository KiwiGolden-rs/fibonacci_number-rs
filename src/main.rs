fn main() {
    for i in 0..=20 {
        println!("Fibonacci ({}) => {}",i, fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}