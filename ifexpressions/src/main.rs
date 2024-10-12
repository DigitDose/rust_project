use std::io;


fn main() {
    let number = 7;

    if number !=0 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }
    
    let mut input : String = String::new();


    io::stdin()
        .read_line( &mut input)
        .expect( "faild to read the line");
    
    let number:i32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please provide a valid numbar ");
            return;
        }
    };
    multi_conditions(number);
    }

    
fn multi_conditions(number: i32){

    if number % 4 == 0 {
        println!("numbar is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4,3, or 2")
    }
}