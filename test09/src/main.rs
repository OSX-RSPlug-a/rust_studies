#![allow(unused, dead_code)]

fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase();
}

fn add_prefix(text: &mut String) {
    //*text = format!("FOO_{text}");  > we can use Implicit Deref 
    text.push_str("_FOO");
}

fn main() {
    
    let mut name = "Dude".to_string();

    to_uppercase(&mut name);
    add_prefix(&mut name);

    println!("{name}");
}