use std::env;

fn main() {

    let args = env::args().collect::<Vec<String>>();

    let value = &args[1];

    if value == "42"  {
        println!("The answer for everething...");
    } else if value == "22" {
        println!("Not the primal answer");
    } else {
        println!("Error! ur answer it`s not accetable");
    };

    println!("{:?}", args);

}
