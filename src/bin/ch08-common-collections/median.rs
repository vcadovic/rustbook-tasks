fn main() {
    let mut v = vec![25, 24, 26, 3];
    let median = median(&mut v);

    match median {
        None => println!("No median"),
        Some(m) => println!("Median is {m}"),
    }
}

fn median(v: &mut Vec<i32>) -> Option<i32> {
    let len = v.len();
    if len == 0 {
        None
    } else {
        v.sort();
        Some(v[(len - 1) / 2])
    }
}