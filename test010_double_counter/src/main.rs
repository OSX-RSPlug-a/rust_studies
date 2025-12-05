pub fn double_counter() -> i32 {
    let mut var1: i32 = 1;

    for _ in 0..5 {
        var1 *= 2;
    }

    var1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_counter() {
        assert_eq!(double_counter(), 32);
    }
}

fn main() {
    let result = double_counter();
    println!("The result of doubling 1 five times is: {}", result);
}