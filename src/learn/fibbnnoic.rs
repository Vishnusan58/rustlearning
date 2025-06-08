use std::io;
pub fn fibbon(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    let mut first = 0;
    let mut second = 1;
    for _ in 2..=num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    second
}

pub fn call_fib() {
    let mut nums = String::new();
    println!("Please enter a number:");
    io::stdin().read_line(&mut nums).expect("Failed to read line");
    let n: i32 = match nums.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };
    let ans = fibbon(n);
    println!("The Fibonacci number at position {} is {}", n, ans);
}

