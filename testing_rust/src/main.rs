use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    //komentarze ciemno zielone
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=1000);
 
    println!("The secret number is {secret_number}");
    
    loop {
    
    println!("Please input your guess.");

    let mut guess: String = String::new();
    


    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    //let guess: u32 = guess.trim().parse().expect("please type a NUMBER !");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess); // ten sposób jest przestarzały w nowszych wersjach rust zastapiony zostal interpolacja zmiennych. może być bezpośrednio wstawiona do stringa bez potrzeby przekazywania jej jako argument

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;  
            }
    }
  }
}

