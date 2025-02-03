use std::io::{stdin, stdout, Write};

fn main() {
    let mut n = String::new();

    let n: i32 = loop {
        n.clear();

        print!("Enter Nth Fibonacci Number: ");
        stdout().flush().expect("Error: Could not flush stdout");

        stdin()
            .read_line(&mut n)
            .expect("Error: Could not read from stdin");

        match n.trim().parse::<i32>() {
            Ok(num) => {
                if num < 1 {
                    println!("Error: Nth Fibonacci Must Be Greater Than or Equal to 1");
                    continue;
                }

                break num;
            }
            Err(_) => {
                println!("Error: Could not parse input");
                continue;
            }
        }
    };

    let res = fib(n);

    println!("Fibonacci No. {n}: {res}");
}

fn fib(n: i32) -> i32 {
    let mut prev = 0;
    let mut cur = 1;

    if n == 1 {
        return prev;
    }

    if n == 2 {
        return cur;
    }

    for _ in 0..n - 2 {
        let tmp = prev + cur;
        prev = cur;
        cur = tmp;
    }

    return cur;
}
