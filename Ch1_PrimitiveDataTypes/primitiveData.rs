fn main(){
    let x: i32 = -42; 
    let y: u64 = 100;

    
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maxiumum value of i32: {}", e);
    println!("Maxiumum value of i64: {}", i);

    let pi: f64 = 3.14;

    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of alpha: {}", letter);

}