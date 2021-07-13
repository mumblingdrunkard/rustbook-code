use std::collections::HashMap;

fn mean(numbers: &[i64]) -> f64 {
    let mut sum = 0.0;
    for &n in numbers {
        sum += n as f64;
    }

    sum / (numbers.len() as f64)
}

fn median(numbers: &[i64]) -> f64{
    let mut numbers = numbers.clone().to_owned();
    numbers.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        (numbers[mid] + numbers[mid - 1]) as f64 / 2.0
    } else {
        numbers[mid] as f64
    }
}

fn mode(numbers: &[i64]) -> i64 {
    let mut occurrences = HashMap::new();

    for n in numbers {
        *occurrences.entry(n).or_insert(0) += 1;
    }

    occurrences
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| **val)
        .expect("Can't get mode of list with no elements")
}

fn main() {
    let data = vec![
        1, 2, 2, 3, 2, 7, 6, 1,
        1, 6, 7, 6, 8, 5, 3, 4
    ];

    let mean = mean(&data);
    println!("Mean: {}", mean);

    let median = median(&data);
    println!("Median: {}", median);

    let mode = mode(&data);
    println!("Mode: {}", mode);
}
