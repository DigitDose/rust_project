use std::io;
pub struct Point<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}
impl<X1, Y1> Point<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<String, String> {
    pub fn new_from_input() -> Self {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("podaj pierwszy param:");
        io::stdin()
            .read_line(&mut input1)
            .expect("nie udało sie odczytac linii");
        let input1 = input1.trim().to_string();

        println!("podaj drugi param:");
        io::stdin()
            .read_line(&mut input2)
            .expect("nie udało sie odczytac linii");
        let input2 = input2.trim().to_string();
        Point {
            x: input1,
            y: input2,
        }
    }
}
