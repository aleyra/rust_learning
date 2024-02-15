// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const can be used in the global scope, and let can only be used in a function

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

// }

// from shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

     // give an error v
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // ok v
    let spaces = "   ";
    let spaces = spaces.len();
}