const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("{}", message());
    println!("Constant: {}", THREE_HOURS_IN_SECONDS);
    shadowing();
    numerical_operations();
    tuple();
}

fn message() -> String {
    String::from("Hello, world!")
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("The length of spaces is: {}", spaces);
}

fn numerical_operations() {
    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("Floored: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);
}

fn tuple() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}, y is: {} and z is: {}", x, y, z);

    println!("The value of 0 is: {}, 1 is: {} and 2 is: {}", tup.0, tup.1, tup.2);
}