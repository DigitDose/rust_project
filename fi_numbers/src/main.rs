use std::io;

fn main() {
    println!("Hello, world!");

    fi_number();

    let mut index:String = String::new();

     
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");



    let number:u32 = match index.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Provide Number!");
            return;
            
        }
    };
        for i in 0..number {
       println!("{},", licz_ciag_fibboncaciego(i));

    }
}
fn fi_number(){

    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

    println!("fi = {} ", phi);
}

fn licz_ciag_fibboncaciego (n:u32)  -> u32 {

    if n == 0 { return 0; } else if n == 1 { return 1; } 
    let mut a = 0; let mut b = 1; 
    for _ in 2 ..= n { 
        let temp = a+b; a = b; b = temp; } return b; 

}
