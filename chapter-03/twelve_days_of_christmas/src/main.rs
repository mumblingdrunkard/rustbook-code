fn main() {
    let verses = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    let ordinals = [
        "zeroth",
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelvth"
    ];

    for day in 1..13 as usize {
        println!("On the {} day of Christmas, my true love sent to me", ordinals[day]);

        // First verse
        if day == 1 {
            println!("{}!", verses[0]);
        }

        // All other cases
        for i in (1..day).rev() {
            println!("{},", verses[i]);
        }

        if day > 1 {
            println!("and {}!", verses[0]);
        }

        println!("");
    }
}
