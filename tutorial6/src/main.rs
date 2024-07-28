use std::io;

fn main() {
    let x: i8 = 125;
    let y: u8 = 10;

    // Can't combine i8 and u8
    // let z = x / y;
    // println!("{}", z);

    let x: f64 = 255.0;
    let y: f64 = 10.0;

    let z = x / y;
    let modulo = x % y;
    println!("z = {}, mod = {}", z, modulo);

    let x: i64 = 255;
    let y: f64 = 10.0;

    // cast
    let z = x / (y as i64);
    println!("z = {}", z);


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("User typed in {}", input);
    println!("As an int: {}", int_input);
}
