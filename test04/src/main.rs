use std::io;

fn main() {

    // String types
    let mut name= String::new();
    name.push_str("Dude ");
    name.push_str("Bug ");
    name.push_str("Slug");
    println!("{:?}", name);

    let n= String::from("Wed");
    println!("{n}");

    let list = ['w', 'e', 'd', 'n', 'e', 's', 'd', 'a', 'y'];
    let conc = String::from_iter(list);
    println!("{conc}");

    let s: String = "Wolfmother".into();
    println!("{s}");

    let x = "Shaman".to_owned();
    println!("{x}");

    let mut text = String::new();
    println!("Insert a text");
    
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading console");

    println!("U`ve inserted {}", text.trim().len());

}
