use std::io;

fn main() {
    let fahrenheit: f32 = loop {
        println!("Please enter temperature in degrees Fahrenheit:");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Unable to read in value");

        match fahrenheit.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);

    println!("The temperature in Celsius is: {}", celsius);
}
