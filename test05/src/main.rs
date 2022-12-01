use std::io;

fn main() {
    
    println!("{:#^150}", "Calculator");

    println!("Insert values ");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    let nums: Vec<i32> = s
        .split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();

    println!("The sum of these number: {}", result);

    println!("{}", "#".repeat(150));

}
