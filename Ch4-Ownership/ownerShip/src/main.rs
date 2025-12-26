//// Ownership
//// 1 - Each  value in Rust has a variable that/s its owner. /////
//// fn main(){
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);

//     println!("Length of '{}' is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }
//// 2 - There can only be one owner at a time. //

    fn main(){
        let s1 = String::from("RUST");
        let s2 = s1;
        
        // println!("{}", s1);
        println!("{}", s2);
    }

//// 3 - When the owner goes out of the scope, the value will be dropped /////
// fn main(){
//         let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
// println!("Length of '{}' is {}", s1, len);

// }

// fn printLost(s: &string){
//     println!("{}", &s1);
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

