const MAX_POINTS: u32 = 100_000; // Valid for entire lifetime of program

fn main() {
    // Variables
    let mut x = 5; // Mutable variable, can be reassigned
    println!("The value of x is: {}", x);

    x = 6; // Mutate variable
    println!("The value of x is: {}", x);


    // Constants
    println!("Value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let x = 5; // New, immutable x, shadows old value of x
    println!("Value of x is: {}", x);

    let x = x + 1;
    println!("Value of x is: {}", x);

    let x = x * 7;
    println!("Value of x is: {}", x);

    let spaces = "    "; // String type
    let spaces = spaces.len(); // Type can change when shadowing
    println!("Number of spaces is: {}", spaces);
}
