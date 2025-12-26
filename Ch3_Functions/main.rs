// Entry point

fn main() {
    hello_world();
    tell_height(232);
    human_id("Sunii", 55, 182.67);

    // Expression block
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // expression (no semicolon)
    };
    println!("Result is: {}", x);

    let y: i32 = add(4, 6);
    println!("The answer to fn 'add' = {}", y);

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your bmi is {:.2}", bmi)
}


// Hoisting - can call fn anywhere in your code //
fn hello_world(){
    println!("Hello, world");
}

// Inpur value //
fn tell_height(height: i32){
    println!("My height is {}", height);
}

// Multiple values //
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is, {}, I am {} years old, and my height is {} cm, ", name, age, height);
}

////////// Expressions and statements //////////
//// Expression: Anything that returns a value.
// 5
// true & false
// add(3,4)
// if condition {value1} and {value2}
// {code}
//----------------------------------------------------

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) ->f64{
    weight_kg / (height_m * height_m)
}

//// Statement: Anything that does not return a value

