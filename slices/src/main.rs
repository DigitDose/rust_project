fn main() {
    let s = String::from("hell o world");
    println!("the value of {}", first_word(&s));

    let len = s.len();
    let slice1 = &s[0..5];

    let slice2 = &s[6..11];
    let slice3 = &s[0..len];
    let slice4 = &s[..];
    let s = "Hello, world!";
    let my_string = String::from("hello world");

    // `first_word` działa na wycinkach `String`, zarówno częściowych, jak i całych
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // `first_word` działa również na referencjach do `String`, które są równoważne całym wycinkom `String`
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` działa na wycinkach literałów stringów, zarówno częściowych, jak i całych
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Ponieważ literały stringów *są* już wycinkami stringów, to działa również bez składni wycinka!
    let word = first_word(my_string_literal);

    println!(
        "Slice 1 {},Slice2 {}, Slice3 {}, Slice4 {}",
        slice1, slice2, slice3, slice4
    );
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
