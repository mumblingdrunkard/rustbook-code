use std::io;

fn main() {
    let alternatives = [
        "Exit",
        "Celsius to Fahrenheit",
        "Fahrenheit to Celsius",
    ];

    loop{
        println!("Please select an alternative:");
        for (i, alternative) in alternatives.iter().enumerate() {
            println!("{}. {}", i, alternative);
        }

        let mut selection = String::new();
        io::stdin().read_line(&mut selection)
            .expect("Failed to read input");

        let selection: usize = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That doesn't look like a number!");
                continue;
            },
        };

        if selection == 0 {
            println!("Thanks for using this program!");
            break
        }

        if selection > 2 {
            println!("That's not a valid alternative!");
            continue;
        }

        println!("Input value in {}",
            match selection {
                1 => "celsius",
                2 => "fahrenheit",
                _ => "[If you see this, an error has occurred]"
            },
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That doesn't look like a number!");
                continue;
            },
        };

        println!("{}", 
            match selection {
                1 => (input * 9.0 / 5.0 + 32.0),
                2 => (input - 32.0) * 5.0 / 9.0,
                _ => 0.0
            }
        );
    }
}
