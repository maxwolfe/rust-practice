use std::io;

// Don't know how to raise errors yet, so fib(0) or less will just return 1 for now
fn fib(n: i128) -> i128 {
    let mut nums = (1, 1);

    for _ in 2..n {
        nums = (nums.1, nums.0 + nums.1);
    }

    nums.1
}

fn main() {
    let n: i128 = loop {
        println!("Please enter number of fibonacci sequence to return:");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("Fibonacci of {}: {}", n, fib(n));
}
