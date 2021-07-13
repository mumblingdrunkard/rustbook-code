#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors
    let v: Vec<i32> = Vec::new();
    println!("Vector is: {:?}", v);

    let mut v = vec![1, 2, 3];
    println!("Vector is: {:?}", v);

    v.push(4);
    v.push(5);

    println!("Vector is: {:?}", v);

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(element) => println!("The third element is {}", element),
        None => println!("There is no third element!"),
    }

    // Below line will panic because of an out of bounds access
    // let does_not_exist = &v[100];
    // println!("does_not_exist = {}", does_not_exist);

    // This will not panic
    {
        let does_not_exist = v.get(100);
        match does_not_exist {
            Some(element) => println!("does_not_exist = {}", element),
            None => println!("Element does not exist"),
        }
    }

    for i in &mut v {
        *i += 20;
    }

    for i in &v {
        println!("{}", i);
    }

    // Vectors can only store values of the same type
    // By utilising variants of the same enum, we can store
    // different data
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(420.69),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", row);
}
