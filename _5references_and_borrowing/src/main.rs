// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References

// References: Enable you to borrow values without taking ownership
    // Immutable Reference
    // Mutable Reference
// To CREATE reference by add "&" to the variable you're reffering to.

// -I- Immutable Reference:
// fn main() {
//     let _x: i32 = 5;
//     let _y: &i32 = &_x;
    
//     println!("Value of _x: {}", _x);
//     println!("Value of _y: {}", _y);
// }

// -II- Mutable Reference:
// fn main() {
//     let mut _x: i32 = 5;
//     let _y: &mut i32 = &mut _x;
//     *_y += 1;
//     *_y -= 3;
    
//     println!("Value of _x: {}", _x);
//     // println!("Value of _y: {}", _y);
// }

//!  Note: We can have only one mutable reference or many mutable references.
//! you can have either one mutable reference to a value or any number of immutable refernces.


//? STRUCT:

    // A data structure that allows you to group multiple fields tgether under one name
    // also used in C & C++

// Demonstration on one mutable reference or many immutable references
fn main() {
    let mut account: BankAccount = BankAccount{
        owner: "Akbarsha".to_string(),
        balance: 70000.79,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money 
    account.withdraw(amount: 300.01);
    
    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}