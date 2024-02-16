// from 3.1
/*
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const can be used in the global scope, and let can only be used in a function

// fn main() {
// 	let mut x = 5;
// 	println!("The value of x is: {x}");
// 	x = 6;
// 	println!("The value of x is: {x}");

//}

// from shadowing
fn main() {
	// let x = 5;
	// let x = x + 1;

	// {
	//	 let x = x * 2;
	//	 println!("The value of x in the inner scope is: {x}");
	// }

	// println!("The value of x is: {x}");

	 // give an error v
	// let mut spaces = "   ";
	// spaces = spaces.len();

	// ok v
	let spaces = "   ";
	let spaces = spaces.len();
}
*/

//from 3.2
/*use std::io;

fn main() {
	// let guess: u32 = "42".parse().expect("Not a number!");

	//from flotting point
	/*{
		let x = 2.0; // f64

		let y: f32 = 3.0; // f32
	}*/

	//from Numeric operations
   /*{
		// addition
		let sum = 5 + 10;

		// subtraction
		let difference = 95.5 - 4.3;
	
		// multiplication
		let product = 4 * 30;
	
		// division
		let quotient = 56.7 / 32.2;
		let truncated = -5 / 3; // Results in -1
	
		// remainder
		let remainder = 43 % 5;
   }*/

   //from The boolean type
   /*{
		let t = true;

		let f: bool = false; // with explicit type annotation
   }*/

   //from The Character Type
   /*{
		let c = 'z';
		let z: char = 'ℤ'; // with explicit type annotation
		let heart_eyed_cat = '😻';
   }*/

   // from The Tuple Type
   /*{
		// let tup: (i32, f64, u8) = (500, 6.4, 1);

		// let tup = (500, 6.4, 1);

		// let (x, y, z) = tup;

		// println!("The value of y is: {y}");

		let x: (i32, f64, u8) = (500, 6.4, 1);

		let five_hundred = x.0;

		let six_point_four = x.1;

		let one = x.2;
   }*/

   //from The Array Type
   /*{
		// let a = [1, 2, 3, 4, 5];

		let months = ["January", "February", "March", "April", "May", "June", "July",
			"August", "September", "October", "November", "December"];

		// let a: [i32; 5] = [1, 2, 3, 4, 5]; // [i32; 5] := array de taille 5 contenant des i32

		// let a = [3; 5]; // <=> let a = [3, 3, 3, 3, 3];

		// let a = [1, 2, 3, 4, 5];

		// let first = a[0];
		// let second = a[1];

		let a = [1, 2, 3, 4, 5];

		println!("Please enter an array index.");

		let mut index = String::new();

		io::stdin()
			.read_line(&mut index)
			.expect("Failed to read line");

		let index: usize = index
			.trim()
			.parse()
			.expect("Index entered was not a number");

		let element = a[index];

		println!("The value of the element at index {index} is: {element}");
   }*/
}*/

// from 3.3
/*fn main() {
	// println!("Hello, world!");

	// another_function();
	// another_function(5);

	// print_labeled_measurement(5, 'h');

	let y = 6; // is a statement

	// let x = (let y = 6); // produce an error because a statement does not return a value

	// let y = {
	// 	let x = 3;
	// 	x + 1 // is an expression so it'll return a value
	// };
	// println!("The value of y is: {y}");

	// let x = five();
	// println!("The value of x is: {x}");

	let x = plus_one(5);
	println!("The value of x is: {x}");
}

/*fn another_function() {
	println!("Another function.");
}*/

fn another_function(x: i32) {
	println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
	println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { // '->' is needed to declare the type of the returned value
	5 // it's a returned value
}

fn plus_one(x: i32) -> i32 {
	x + 1
}*/

// from 3.5

fn main() {
	// let number = 7;

	// if number < 5 { // condition must be a bool or else we've got an error
	// 	println!("condition was true");
	// } else {
	// 	println!("condition was false");
	// }

	// if number % 4 == 0 {
	// 	println!("number is divisible by 4");
	// } else if number % 3 == 0 {
	// 	println!("number is divisible by 3");
	// } else if number % 2 == 0 {
	// 	println!("number is divisible by 2");
	// } else {
	// 	println!("number is not divisible by 4, 3, or 2");
	// }

	// let condition = false;
	// let number = if condition { 5 } else { 6 }; // here the 2 expressions must be the same type or we get an error
	// println!("The value of number is: {number}");
	
	// loop { // infinite loop because no "break"
	// 	println!("again!");
	// }

	// let mut counter = 0;
	// let result = loop {
	// 	counter += 1;

	// 	if counter == 10 {
	// 		break counter * 2; // ';' is technically optional, "break" is very similar to "return"
	// 	}
	// };
	// println!("The result is {result}");

	// let mut count = 0;
	// 'counting_up: loop {
	// 	println!("count = {count}");
	// 	let mut remaining = 10;

	// 	loop {
	// 		println!("remaining = {remaining}");
	// 		if remaining == 9 {
	// 			break;
	// 		}
	// 		if count == 2 {
	// 			break 'counting_up;
	// 		}
	// 		remaining -= 1;
	// 	}

	// 	count += 1;
	// }
	// println!("End count = {count}");

	// let mut number = 3;
	// while number != 0 {
	// 	println!("{number}!");

	// 	number -= 1;
	// }
	// println!("LIFTOFF!!!");

	// let a = [10, 20, 30, 40, 50];
	// let mut index = 0;
	// while index < 5 { // like 'for' after this
	// 	println!("the value is: {}", a[index]);

	// 	index += 1;
	// }

	// let a = [10, 20, 30, 40, 50];
	// for element in a { // like "while" above
	// 	println!("the value is: {element}");
	// }

	for number in (1..4).rev() { // in range (inclued..exclued)
		println!("{number}!");
	}
	println!("LIFTOFF!!!");
}

