fn main() {
    let mut s = String::from("hello");
    
    change(&mut s); // mutable borrow
    println!("s is now: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}