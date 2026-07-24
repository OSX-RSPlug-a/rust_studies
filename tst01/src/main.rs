
pub enum Shape {
    Circle(f64),
    Square(f64),
}


pub trait Speak {
    fn speak(&self) -> String;
}


impl Speak for Shape {
    fn speak(&self) -> String {
        match self {
            Shape::Circle(r) => format!("The circle with radius {r}!"),
            Shape::Square(s) => format!("The square with side {s}!"),
        }
    }
}


fn main() {
    let circle = Shape::Circle(5.0);
    println!("{}", circle.speak());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Shape::Circle(5.0);
        assert_eq!(circle.speak(), "I am a circle with radius 5!");
    }
}