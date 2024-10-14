fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let (num, den) = remove_irrationality(3.0, 7.0 * 2.0_f64.sqrt());

    println!("Nowy licznik: {}, Nowy mianownik: {}", num, den);
    let mut num = 3.0;
    let mut den = 7.0 * 2.0_f64.sqrt();
    remove_irrationality_with_ref(&mut num, &mut den);
    println!("Nowy ułamek to {}/{} ", num, den);

    let rect1 = Rectangle {
        width: 60,
        heigth: 60,
    };
    println!(
        "The area1 of the rectangle is {} square pixels",
        area1(&rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn remove_irrationality(numerator: f64, denominator: f64) -> (f64, f64) {
    if denominator.fract() != 0.0 {
        let conjugate = denominator;
        let new_numerator = numerator * conjugate;
        let new_denominator = denominator * conjugate;
        (new_numerator, new_denominator)
    } else {
        (numerator, denominator)
    }
}
fn remove_irrationality_with_ref(ref_num: &mut f64, ref_den: &mut f64) {
    if ref_den.fract() != 0.0 {
        let conjugate = *ref_den;
        *ref_num *= conjugate;
        *ref_den *= conjugate;
    }
}

struct Rectangle {
    width: u32,
    heigth: u32,
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}
