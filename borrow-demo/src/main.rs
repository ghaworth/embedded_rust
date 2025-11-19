fn main() {
    let mut value = 42; // owned on stack

    let read_borrow = &value;
    println!("Borrowed value: {}", read_borrow); // ok

    change_value(&mut value); // borrow mutable
    println!("Owned value now: {}", value); // still owns it
}

fn change_value(v: &mut i32) {
    *v += 1; // * dereferences to change
}
