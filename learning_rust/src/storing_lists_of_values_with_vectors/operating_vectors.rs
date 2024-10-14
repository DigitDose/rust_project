pub fn create_vector() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
pub fn update_vector(v: &mut Vec<i32>) {
    v.push(5);
    v.push(6);
    v.push(8);
}

pub fn find_the_largest_number<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.is_empty() {
        return None;
    }
    let mut largest = &s[0];
    for item in s {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
