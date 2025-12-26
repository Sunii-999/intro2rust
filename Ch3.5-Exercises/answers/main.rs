fn main(){
    //--------------------------------------
    // Exercise 1
    let temperature : i32 = -2;
    let age : u32 = 56;
    let price : f64 = 2.56;
    let is_logged_in : bool = true;
    let firstchar: char = 'S';

    println!("It is {} degrees outside, I am {}, the price {}, I am logged in {}, my name starts with an {}", temperature, age, price, is_logged_in, firstchar);

    //--------------------------------------
    // Exercise 2
    let a : i32 = 2112;
    let b : i64 = 4565534;

    println!("a = {}, b = {}", a,b,);

    // i64 holds larger numbers

    //--------------------------------------
    // Exercise 3

    let array : [i32;5] = [1,2,3,4,5];
    println!("Whole array: {:?}", array);
    println!("First item: {:?}", array[0]);
    println!("Last item: {:?}", array[4]);

    // index  does not exist and will throw error, array count starts from 0

    //--------------------------------------
    // Exercise 4
    let human: (&str, i16, f32) = ("Sunii", 30, 30.2);

    println!("Whole of tupple: {:?}", human);

    //// bonus
    let bonus_tuples: (&str, i16, f32) = ("Sunii", 30, 30.2);
    println!("My name is {}, I am {} years old, I am {} m tall", bonus_tuples.0, bonus_tuples.1, bonus_tuples.2);


    //--------------------------------------
    // Exercise 5

    let weight: f64 = 80.3; // in kg
    let height: f64 = 1.65; // in meters

    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi); // print BMI with 2 decimal places

    // Correct BMI categorization
    if bmi < 18.5 {
        println!("Category: Underweight");
    } else if bmi >= 18.5 && bmi < 25.0 {
        println!("Category: Normal weight");
    } else if bmi >= 25.0 && bmi < 30.0 {
        println!("Category: Overweight");
    } else {
        println!("Category: Obese");
    }
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}