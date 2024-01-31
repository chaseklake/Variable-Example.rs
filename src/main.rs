// Example of const in rust. "_" used at beginning of name to remove "unused variable" warning.
const _THREE_HOURS_IN_SECONDS: i32 = 60*60*3;

// Main function: Examples of mutable an immutable variables.
fn main() {
    // How a mutable variable is defined:
    let mut x = 5;
    println!("The value of x is: {x}");

    // Modifying the variable:
    x = 6;
    println!("The value of x is: {x}");

    // Re-defining (referred to as "Shadowing") a variable lets us make it immutable.
    let x = x + 1;
    println!("The value of x is: {x}"); // Prints 7
    
    // ERROR: Now that x is immutable, the following will produce an error.
    // x = 6;
    // println!("The value of x is: {x}");

    // An example of how variables are maintained in scope:
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}"); // Prints 14
    }
    println!("The value of x in the outer scope is : {x}"); // Prints 7
}
