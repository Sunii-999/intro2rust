// References and Borrowing
// Safety and Performance

// fn main() {
    // let mut _x : i32 = 5;
// 
    // let _r : &mut i32 = &mut _x;
// 
    // *_r += 1;
    // *_r -= 3;
// 
    // println!("Value of _x = {}", _x);
    // println!("Value of _r = {}", _r);
// 
    //// --------- IMPORTANT ----------------
    //// you can only have one mutable reference or many immutable references
    //// see demostration 1
//  }
// 

// Demostration 1

fn main() {
    let mut account : BankAccount = BankAccount{
        owner: "Sunii".to_string(),
        balance: 150.54,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(54.65);

    // Check balance again
    account.check_balance();
 }

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
}

