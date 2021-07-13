fn main() {
    let a = 7;
    let b = 6;
    let c = another_function(a, b);

    println!("Value of c is: {}", c);

    function_that_prints();
}

fn another_function(a: i32, b: i32) -> i32 {
    a + b
}

fn function_that_prints() {
    println!("Hello from outside main!");
}
