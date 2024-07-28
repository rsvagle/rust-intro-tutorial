fn main() {
    println!("Hello, world!");

    let cond = 2 < 4;
    println!("{}", cond);

    // this will error
    // let cond2 = 2 < 4.5;
    // println!("{}", cond2);

    let cond3 = (2 as f32) < 4.5;
    println!("{}", cond3);

    // AND - &&
    // OR - ||
    // NOT - !

    // Operators in order - ! && ||

    let cond4 = true && cond;
    println!("{}", cond4);

    let food = "pickle";

    if food == "cookie" {
        println!("I like cookies too!");
    }
    else {
        println!("I like a cookie more than a {}!", food);
    }
}
