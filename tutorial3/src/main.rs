fn main() {
    const SECONDS_IN_MINUTES: u32 = 60;

    let x = 4;
    println!("x is {}", x);

    {
        let x = 2;
        println!("x is {}", x);
    }

    let x = x + 1;
    println!("x is {}", x);

    println!("SECONDS_IN_MINUTES is {}", SECONDS_IN_MINUTES);
}
