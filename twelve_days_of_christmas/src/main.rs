fn main() {
    let presents = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swiming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];
    let days = [
        (1, "first"),
        (2, "second"),
        (3, "third"),
        (4, "fourth"),
        (5, "fifth"),
        (6, "sixth"),
        (7, "seventh"),
        (8, "eighth"),
        (9, "ninth"),
        (10, "tenth"),
        (11, "eleventh"),
        (12, "twelfth"),
    ];

    for day in days.iter() {
        println!("On the {} day of Christmas\nMy true love gave to me", day.1);

        for num in (0..day.0).rev() {
            match day.0 {
                1 => println!("A partridge in a pear tree"),
                _ => println!("{}", presents[num]),
            };
        }
        println!();
    }
}
