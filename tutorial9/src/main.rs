// Stack and Heap | Memory management

fn main() {
    println!("Hello, world!");
}

// Stack must be things of fixed size
fn fixed_size(){
    let x = 2; // Stack is x = 2
    let y = x; // Stack is x = 2, y = 2
} // y pops off, then x pops off

fn example(){
    let a = "true";
    let b = false;
} // b pops off first, a pops off second

fn nested_func(){
    let x = 2; // Stack is x = 2
    let y = x; // Stack is x = 2, y = 2

    example(); // Stack is x = 2, y = 2, a = "true", b = false

    // Stack is x = 2, y = 2
}

fn nest_func_with_params(){
    let x = 2; // Stack is x = 2
    let y = x; // Stack is x = 2, y = 2

    add_nums(x,y); // Stack is x = 2, y = 2, x = 2, y = 2
}

fn add_nums(x: i32, y: i32) -> i32 {
    x + y
}

// Heap needs to find space, so you have to look through it
// Once something is on the heap, an entry in the stack is created with the pointer to the mem address

// Heap
fn heap_main() {
    let x = 2; // Stack is x = 2
    let string = String::from("hello"); // Heap contains "hello" at some address, Stack is x = 2, string = [[MEM ADDRESS]]
}
