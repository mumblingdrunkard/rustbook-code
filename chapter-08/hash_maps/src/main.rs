use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![
        "Blue".to_string(),
        "Yellow".to_string()
    ];

    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter()
        .zip(initial_scores.iter())
        .collect();

    println!(
        "Blue team score is: {}",
        match scores.get(&"Blue".to_string()) {
            Some(score) => score.to_string(),
            None => "[undefined]".to_string()
        }
    );

    println!(
        "Red team score is: {}",
        match scores.get(&"Red".to_string()) {
            Some(&score) => score.to_string(),
            None => "[undefined]".to_string()
        }
    );

    for team in &teams {
        println!(
            "{}: {}",
            team,
            match scores.get(&team) {
                Some(&score) => score.to_string(),
                None => "[undefined]".to_string(),
            }
        );
    }

    // HashMap Ownership
    let mut map = HashMap::new();

    {
        let field_name = String::from("Favourite colour");
        let field_value = String::from("Red");
        map.insert(field_name, field_value); // Takes ownership
        // field_name and field_value are now invalid
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // Overwriting:
    scores.insert(String::from("Blue"), 20);

    println!("{:#?}", scores);

    // Only inserting a value if the key has no value
    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);

    // Updating a value based o nthe old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("text: '{}'", text);

    println!("{:#?}", map);

    // NOTE: You can specify your own hashing function if you
    // find the default one to be too slow
}
