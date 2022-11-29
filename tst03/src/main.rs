fn main() {
    // type ex u16 - but i32 it`s default
    let x = 5_u16;
    let y = x + 20_u16;
    println!("{:?}", y);

    // tuple ex
    let mut numbers   = (1, 2, 3);
    let (_a, _b, _c) = numbers;
    numbers.2 = 42;
    println!("{:?}", numbers);
    println!("{:?}", _c);

    // array
    let group = [743.1, 505.2, 42.0];
    println!("{:?}", group);
    println!("{:?}", group[2]);
    // slice ex
    println!("{:?}", &group[0..2]);

}
