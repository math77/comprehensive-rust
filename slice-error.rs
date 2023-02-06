fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; //borrow happens here
    println!("s: {s:?}");

	a[3] = 7; //error: cannot assign to `a[_]` because it is borrowed
	
	println!("s modified: {s:?}");
}
