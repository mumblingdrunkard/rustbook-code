fn main() {
    // Conditionals
    let number = 3;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    // Conditional assignment
    let message = if number < 5 {
        "Condition was true"
    } else {
        "Condition was false"
    };

    println!("Message: {}", message);

    // Loops

    // Loop, but bad
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        println!("`i` is: {}", i);
        i = i + 1;
    }

    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2;
        }
    };

    println!("Result is: {}", result);

    // Conditional looping
    let mut i = 3;
    while i != 0 {
        println!("{}!", i);

        i -= 1;
    }

    println!("LIFTOFF!!!");

    // For loop
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // For loop with range
    for number in (1..10).rev() {
        println!("{}!", number);
    }

    println!("Liftoff!");
}
