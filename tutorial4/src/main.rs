fn main() {
    println!("Hello, world!");

    // Data types
    let x: i32 = 2;
    let y: u32 = 90;
    println!("x is {}", x);
    println!("y is {}", y);

    let z: f32 = 125.5;
    println!("z is {}", z);

    let true_or_false: bool = false;
    println!("true_or_false is {}", true_or_false);

    let letter: char = 'a';
    println!("letter is {}", letter);

    let mut tup: (i32, bool, char) = (1, true, 's');
    let tup2: (i8, bool, char) = (1, true, 's');
    println!("tup index 0 is {}", tup.0);
    println!("tup index 1 is {}", tup.1);
    println!("tup index 2 is {}", tup.2);

    tup.0 = 10;
    println!("tup index 0 is {}", tup.0);

    let arr = [1,2,3,4,5];
    println!("arr index 0 is {}", arr[0]);

    let mut array: [i32;3] = [1,2,3];
    println!("arr index 0 is {}", array[0]);
}
