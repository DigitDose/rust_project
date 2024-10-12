fn main() {
    println!("Hello, world!");

    another_fn(5,'h');

    let x = five();

    println!("The value of x is {x}");

    let y = plus_one(5);
    
    println!("Function plus_one with arg5 {y}")
}
fn another_fn(x:i32, unit_label: char) {
    println!("Measurement is :{x}{unit_label}");
}
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {

    x + 1

}