fn main() {
    let x = 5; //immuatability bu default
    // x = 10; // error - can't mutate
    print_value(&x); // borrow read-only
    println!("x is still {}", x);
}

fn print_value(y: &i32) {
    println!("Value: {}", y);
    *y = 10; // error - can't mutate borrowed immutable
}