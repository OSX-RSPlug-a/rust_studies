#![allow(unused, dead_code)]

// fn say_hello(text: &String) {  // if we use the reference & - it can be borrow
fn say_hello(text: String) {
    println!("Hello, {text}")  
}

// fn say_goodbye(text: &String) {  
fn say_goodbye(text: String) {
    println!("Goodbye, {text}")
}

fn main() {
    
    let name = "Dude".to_string();

    //say_hello(&name); // use & to borrow value from var name
    //say_goodbye(&name);
    say_hello(name.clone()); // use .clone() to use mem heap
    say_goodbye(name);

}
