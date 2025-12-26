// Compound Data Types
// arrays, tuples, slics and strings (slice string)

fn main(){

    /////////////////     Arrays    /////////////////    
   
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    // let mix =[1,2,"apple", true];
    // println!("Mix Array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("First Fruit: {}", fruits[0]);
    println!("Second Fruit: {}", fruits[1]);
    println!("Third Fruit: {}", fruits[2]);

    /////////////////     Tuples    /////////////////    
    
    // let human = ("Sunii", 30, false); <- also possible
    // let human: (String, i16, bool) = ("Sunii".to_string(), 30, false); <- define string in values
    let human: (&str, i16, bool) = ("Sunii", 30, false);
    println!("Human: {:?}", human);

    
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);
    
    /////////////////     Slices    /////////////////    

    
    let number_slices :&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices :&[&str] = &["Lion", "Zebra", "Giraffe","Hippo"];
    println!("Madagascar main characters: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Zebra".to_string(), &"Giraffe".to_string(), &"Hippo".to_string()];
    println!("Madagascar main characters: {:?}", book_slices);

    /////////////////     Strings Vs String Slices (&str)    /////////////////    
    /////////////////     Strings [growable, mutable, owned string type]    /////////////////    

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("YEAH");
    println!("Stone Cold Says {}", stone_cold);

    let string: String = String::from("Hello, World");
    // let slice: &str = &string;
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}

/// cannot get value outside of scope
fn print(){ 
    println! ("Slice: {}", slice);
}