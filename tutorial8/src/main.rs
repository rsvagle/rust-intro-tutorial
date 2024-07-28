fn main() {
    println!("Hello, world!");
    test_one();
    add_nums(20, 30);

    // statement with an expression
    let number = {
        let x = 3;
        x + 1 // Cant have a semi-colon here
    };

    println!("Number is {}", number);

    // function with return value
    let result = add_numbers(2,3);
    println!("Result is {}", result);

    // function with return value
    let result = add_numbers2(2,3);
    println!("Result is {}", result);

    // function with return value
    let result = add_numbers3(2,3);
    println!("Result is {}", result);
}

fn test_one() {
    println!("This is the test_one function");
}

fn add_nums(x: i32, y: i32) {
    println!("The sum is {}", x + y);
}

// expressions vs statements
// statement: let x = 20; -- cant do let x = (let y = 20); In python x = y = 20 is valid
// expression: returns a value: 2 + 3, 2 < 3, etc.

// function with a return
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

// function with a return version 2
fn add_numbers2(x: i32, y: i32) -> i32 {
    return x + y;
}

// like add numbers, just end the function with the value to return
fn add_numbers3(x: i32, y: i32) -> i32 {
    let result = x + y;

    result // simply end with the expression
}