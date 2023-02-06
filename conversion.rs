/*
 
Reminder:

The Rust integer types all implement the From<T> and Into<T> traits to let us convert between them. The From<T> trait has a single from() method and similarly, the Into<T> trait has a single into() method. Implementing these traits is how a type expresses that it can be converted into another type

The standard library has an implementation of From<i8> for i16, which means that we can convert a variable x of type i8 to an i16 by calling i16::from(x). Or, simpler, with x.into(), because From<i8> for i16 implementation automatically create an implementation of Into<i16> for i8.

*/


fn multiply(x: i16, y: i16) -> i16 {
	x * y
}

fn main() {
	let x: i8 = 15;
	let y: i16 = 1000;

	println!("{x} * {y} = {}", multiply(x.into(), y)); //convert if x is bool, u8, i8
	//or
	println!("{x} * {y} = {}", multiply(i16::from(x), y));
}
