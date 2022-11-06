use std::io;

fn main() {
    println!("Enter the temp in celsius:");

    let mut c = String::new();

    io::stdin().read_line(&mut c).expect("Error reading input.");

    let c: f64 = c.trim().parse().expect("Please type a number!");

    let f = (c * 1.8) + 32.0;

    println!("Fahrenheit: {:.1}", f);
}
