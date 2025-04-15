use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 2, 8, 9, 10];
    let m = mode(&v);
    match m {
        None => println!("No mode"),
        Some(m) => println!("Mode is {m}"),
    }
}

fn mode(v: &Vec<i32>) -> Option<i32>{
    let len = v.len();
    if len == 0 {
        None
    } else {
        let mut dict = HashMap::new();
        for &num in v {
            *dict.entry(num).or_insert(0) += 1;
        }

        let result = dict.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(num, _)| num);

        result
    }
}