fn main() {
    ownership_transfer();
    cloning_data();
    primitive_types();
}

// EXAMPLE OF SIMPLE OWNERSHIP TRANSFER
fn ownership_transfer() {
    let s1 = String::from("hello"); //Rezerwacja pamięci na stercie (heap) na , s1 przyjmuje ownership i rezerwuje na stosie(stack) i przechowuje wkaznik, dlugosc, pojemnosc
    let s2 = s1; // Ownership of s1 is tranfereed to s2 (przeniesienie wlaśnosci)
    let s = String::from("hell");
    //println!("{}",s1); //Error!!!  s1 no longer own the value (s1 dłużej nie ma własnosci nie ma ownership, teraz ma 2)
    println!("{}", s2); // THIS IS THE PROOF
    takes_ownership_with_refference(&s);

    println!("{}", s); //works because ownership was not transferred yet
                       //example of taking ownership
    takes_ownership(s);

    //println!("{}",s1); // this will not run, because s1 is no longer owns the value bali

    let s3 = gives_ownership(); // function returns ownership to s1
    println!("{}", s3);
    let s4: String = String::from("hell2");
    let s5: String = takes_and_gives_back(s4); //s4 looses  ownership s5 gains it,
    println!("{}", s5);
}
fn cloning_data() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // data is cloned, not moved, so S1 have ownership and S2 have ownership too

    println!("{}", s1); //works

    println!("{}", s2); //works
}

fn primitive_types() {
    // primitive types are always clone, so we dont mind.

    let x = 5;
    let y = x;
    println!("x: {} y:{}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// Solution of takes_ownership example, pass a reference to the function
fn takes_ownership_with_refference(some_string: &String) {
    println!("{}", some_string);
}

// Returning ownership from functions

fn gives_ownership() -> String {
    let some_string = String::from("hell4");

    some_string // ownership is retured
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string //ownership is passed and returned
}
