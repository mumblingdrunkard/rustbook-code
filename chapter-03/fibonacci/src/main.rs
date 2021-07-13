fn main() {
    let n = 1000;
    #[allow(unused)]
    let (mut a, mut b, mut c) = (1, 0, 0);

    for i in 1..n {
        println!("The {}{} fibonacci number is: {}", i, ordinal(i), c);

        c += 1;
    }
}

fn ordinal(n: u64) -> String {
    let n = n % 100;
    if n < 20 && n > 10 {
        "th".to_string()
    } else {
        match n % 10 {
            1 => "st".to_string(),
            2 => "nd".to_string(),
            3 => "rd".to_string(),
            _ => "th".to_string()
        }
    }
}
