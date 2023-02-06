
//error:  `x` does not live long enough

fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
	//'x' is dropped here while still borrowed
    println!("ref_x: {ref_x}");
}
