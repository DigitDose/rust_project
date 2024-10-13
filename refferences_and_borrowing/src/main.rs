fn main() {
    let s1 = String::from("hell o string");
    let len = calculate_length(&s1); // we pass reff to String
    println!("the Length og {s1} is {len}"); // s1
                                             //if we want to change something  we must make it mutable like
    let mut s1_mut = String::from("hello");
    {
        let r2 = &mut s1_mut; //this will work
    }

    let r1 = &mut s1_mut; //if we try to use the ref of mut more then 1 time at once, we cannot do this.
                          // let r2 = &mut s1_mut; // this will not run because we can borrow one time, if we want to use this we must make scope

    change(&mut s1_mut); // if we dont use mut, it wont compile
    let mut s3 = String::from("hello");

    let r4 = &s3; // no problem
    let r5 = &s3; // no problem
    println!("{r4} and {r5}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s3; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

//here is a example how to change something we borrow
fn change(some_string: &mut String) {
    some_string.push_str(",World");
}
//This will not work
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!
//But it works when we remove ref
fn not_dangle() -> String {
    let s = String::from("hello");
    s
}
