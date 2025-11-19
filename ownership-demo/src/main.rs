fn main() {
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1.clone();                     // s2 takes ownership
    println!("{}", s1);           // ERROR! s1 is moved
    println!("{}", s2);              // OK — s2 owns it now
}