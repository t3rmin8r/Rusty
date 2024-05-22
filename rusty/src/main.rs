
/*
This is a comment
But the way this is layed out
I can multi-line comments instead 
of the "//"
*/

fn main() {
    println!("Hello, world!"); //This can be used for inline comments
   // println!('Hello, world!'); //This is not allowed

//NOTES:
    // Scalar types: int, float, boolean, char
    //Unsigned (Never Negative u8, u16, u32, u64, u128, usize) 
    //&
    //Signed (Can be negative and positive)(i8, i16, i32, i64, i128, isize)
    // ***   DEFAULT is i32   ***

/*     println!("Max size of a u32: {}", u32::MAX);
    println!("Max size of a u32: {}", u64::MAX);
    println!("Max size of a u32: {}", i32::MAX);
    println!("Max size of a u32: {}", i64::MAX); */

    //floats - f32, f64 (Default is f64 but slower) These are anything with a decimal with a whole number

/*     println!("Max size of a f32: {}", f32::MAX);
    println!("Max size of a f64: {}", f64::MAX); */


    /*
    boolean = true or false 
    represented as bool
     */

    /* 
    character - char - 4 bytes
    */

    // Keep in mind that variables in Rust are immutable unless ....
/*     let mut hello = "Hello, world!";
    println!("{}", hello);

    hello = "Hello, Again!";
    println!("{}", hello); */

let x = 5;
let y = 6;
println!("Math in Rust: {} + {} = {}", x, y, x+y);

//Constants
/*
const need ALL CAPS - 
*/
const NUMBER: i32 = 17;
println!("{}", NUMBER);


}
