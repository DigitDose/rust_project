mod storing_lists_of_values_with_vectors;
mod trait_in_structs;
use crate::storing_lists_of_values_with_vectors::operating_vectors::{
    create_vector, update_vector,
};
use storing_lists_of_values_with_vectors::operating_vectors::find_the_largest_number;
use trait_in_structs::in_struct::Point;
fn main() {
    println!("Hello, world!");
    create_vector();
    let mut vector_ex: Vec<i32> = vec![1, 2, 3];
    update_vector(&mut vector_ex);
    println!("{:?}", vector_ex);
    let unch_v = vec![1, 2, 3, 4, 5];

    let thrid: Option<&i32> = unch_v.get(2);
    match thrid {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    let mut v = String::new();
    let o_string = "hello";
    v.push_str(&o_string);
    println!("{}", v);

    let v = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", find_the_largest_number(&v));

    let number_list = vec![324, 234, 23, 425, 4, 68, 1];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("the largest number is {largest}");

    let point = Point::new_from_input();
    println!("Point Coordinates:({},{})", point.x, point.y);
}
