fn main() {
    let n = 10;
    let fib = nth_fibonacci(n);
    println!("{n}th Fibonacci number is {fib}");
}

fn nth_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;

            for _ in 2..n {
                let temp = curr;
                curr += prev;
                prev = temp;
            }

            curr
        }
    }
}