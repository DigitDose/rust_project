fn main() {

    let tup2: (i32, f64, u8) = (534,6.2,45); 

    let tup = (500, 6.4, 1);

    let (_x,y,_z) = tup;

    let five_hundret_thirty_four = tup2.0;

    let six_point_two = tup2.1;

    let fourty_five = tup2.2;

    println!("The value of y is: {y}");
    
}