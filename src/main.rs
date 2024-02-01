use std::io;

// Example of const in rust. "_" used at beginning of name to remove "unused variable" warning.
const _THREE_HOURS_IN_SECONDS: i32 = 60*60*3;

// Main function: Demonstrates each variable type
fn main() {
    loop {
        print_pretty(print_options);
        println!("Select an option: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        match input {
            1 => print_pretty(shadowing_example),
            2 => print_pretty(integer_types),
            3 => print_pretty(float_types),
            4 => print_pretty(boolean_type),
            5 => print_pretty(char_type),
            6 => print_pretty(tuple_type),
            7 => print_pretty(array_type),
            8 => break,
            _ => {
                println!("Invalid input");
            }
        }
    }
}

// Prints all the options
fn print_options() {
    println!("1. Shadowing Example");
    println!("2. Integer types");
    println!("3. Floating Point types");
    println!("4. The Boolean type");
    println!("5. The Character type");
    println!("6. The Tuple type");
    println!("7. The Array Type");
    println!("8. Quit");
}

// Wraps the print functions to add spacing
fn print_pretty(print_fn: fn()) {
    println!("");
    print_fn();
    println!("");
}

// Demonstrates shadowing variables and variable scope
fn shadowing_example() {
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

// This function prints the types of integers
fn integer_types() {
    println!("8 bit:    i8 (signed),    u8 (unsigned)");
    println!("16 bit:   i8 (signed),    u8 (unsigned)");
    println!("32 bit:   i8 (signed),    u8 (unsigned) [default]");
    println!("64 bit:   i8 (signed),    u8 (unsigned)");
    println!("128 bit:  i8 (signed),    u8 (unsigned)");
    println!("arch:     isize (signed), usize (unsigned)\n"); // "Size" depends on OS architecture, e.g. x64 architecture is 64 bits.

    println!("Literals:");
    println!("Decimal    (3_14u32): {}", 3_14u32);
    println!("Hex           (0xff): {}", 0xff);
    println!("Octal         (0o77): {}", 0o77);
    println!("Binary (0b1111_0000): {}", 0b1111_0000);
    println!("Byte          (b'A'): {}", b'A'); // u8 only
}

// This function prints the types of floating point values
fn float_types() {
    let x = 2.0;    // f64
    let y: f32 = 3.14;   // f32
    println!("The value of the f64 variable x is: {x}");
    println!("The value of the f32 variable y is: {y}\n");
    println!("Floats are always unsigned and f64 is roughly twice as accurate as f32 (IEEE standard) with similar performance.");
}

// This function describes and demonstrates a boolean value 
fn boolean_type() {
    println!("Only true (1) or false (0).");
    let t = true;
    let f = false;
    println!("True: {}\nFalse: {}", t, f); // using multi-variable string concatenation
}

// This function describes the char type
fn char_type() {
    println!("Single character variable, the most primitive alphabetic type");
    let c = 'c';
    let emoji = '😻'; // copied and pasted from the web, but has to do with UTF-8 encoded text.
    println!("c:     {}", c);
    println!("emoji: {}", emoji);
}

// This function demonstrates the tuple type
fn tuple_type() {
    println!("The first compound type, a tuple groups a variety of variables into one.");
    let tup = (69, 3.14, b'1'); // Defining a tuple. Note that once defined, the size cannot be increased.
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2); // accessing the tuple with property calls
    
    // Accessing a layered tuple requires accessing 2nd order properties:
    let layered_tup = (tup, 'c'); // This does NOT increase the size, it puts a tuple in another tuple.
    println!("Layered tuple: ({}, {}, {}), {}", layered_tup.0.0, layered_tup.0.1, layered_tup.0.2, layered_tup.1);

    // Using pattern matching to extrapolate values from the tuple
    let (x, y, z) = tup; 
    println!("After destructuring the tuple into x, y, z:");
    println!("x = {x}, y = {y}, z = {z}");

    // Learned this gem by reading the compiler error of {tup} in println!()
    println!("tuple read using functional access method (:?): {:?}\n", tup);

    // The empty value is called unit. This is returned by functions which don't return anything else implicitly.
    let unit = ();
    fn void() {}

    println!("unit: {:?}", unit);
    println!("Void returns unit: {}", void() == unit);
}

// This function demonstrates the array type
fn array_type() {
    println!("A fixed length collection of objects with the same type.");
    let arr = [1, 2, 3, 4, 5];
    println!("arr = {:#?}\n", arr); // {:#?} is fancy printing of a collection

    // To explicitly set arrays, type "let arr: [i32, 5] = [1, 2, 3, 4, 5];" where 5 is the size of the array.

    // Sets an array as a collection of the same value (here is 5 threes);
    let arr = [3; 5];
    println!("arr of threes = {:#?}\n", arr);

    let arr = [1, 2, 3, 4, 5]; // Resetting the array
    println!("arr[3] = {}", arr[3]); // Accessing an element of an array. Since arrays are 0 indexed, this will get "4"

    play_with_arrays(arr);
}

fn play_with_arrays(arr: [i32; 5]) {
    loop {
        // The following code was stolen from the text book: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type
        // It was edited slightly.
        println!("Array: {:?}", arr);
        println!("Please enter an array index. Enter 5 or greater to quit.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, defaulting to 0");
                0
            }
        };

        let element = arr[index];

        println!("The value of the element at index {index} is: {element}");
    }
}