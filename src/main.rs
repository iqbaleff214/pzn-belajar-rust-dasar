fn main() {
    println!("Hello, world!");
    print!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, world!");
}

#[test]
fn variable_test() {
    let name = "M. Iqbal Effendi";
    println!("Hello, {}!", name);
}

#[test]
fn mutable_test() {
    let mut name = "M. Iqbal Effendi";
    println!("Hello, {}!", name);

    name = "Alia Putri Safitri";
    println!("Hello, {}!", name);
}

#[test]
fn static_type_test() {
    let mut name = "M. Iqbal Effendi";
    // name = 12; // Type mismatch [E0308] expected `&str`, but found `i32`

    println!("Hello, {}!", name);
}

#[test]
fn shadowing_test() {
    let name = "M. Iqbal Effendi";
    println!("Hello, {} as string!", name);

    let name = 214;
    println!("Hello, {} as number!", name);
}

#[test]
fn comment_test() {
    /*
        Multiline comments
        Do whatever you want
    */
    println!("Hello, world!"); // One-line comment
}

/*
    DATA TYPE
    - Scalar: single value (integer, float, boolean, char)
    - Compound: multiple value (tuple, array)
*/

#[test]
fn explicit_test() {
    let name: &str = "M. Iqbal Effendi";
    let age: u8 = 25;
    
    println!("My name is {}, I'm {} years old.", name, age);
}

/*
    NUMBER DATA TYPe
    integer
    - i8
    - i16
    - i32 (default)
    - i64
    unsigned integer
    - u8
    - u16
    - u32
    - u64
    float
    - f32
    - f64 (default)
    usize (based on operating system)
    - isize
    - usize
 */

#[test]
fn number_test() {
    let a: i8 = 10;
    let b: f32 = 2.5;
    
    println!("{} is integer and {} is float.", a, b);
}

#[test]
fn number_conversion_test() {
    let a: i8 = 10;
    println!("{} is integer.", a);
    let b: i16 = a as i16;
    println!("{} is integer.", b);
    let c: i32 = b as i32;
    println!("{} is integer.", c);
    let d: f32 = c as f32;
    println!("{} is float.", d);
    let e: f64 = d as f64;
    println!("{} is float.", e);
    let f: i8 = e as i8;
    println!("{} is integer.", f);
    
    let x: i64 = 1000000000000000000;
    println!("{} as i64.", x);
    let y: i8 = x as i8; // overflow
    println!("{} as i8.", y);
}

#[test]
fn numeric_operation_test() {
    let a = 10;
    let b = 20;
    
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
    let d = a - b;
    println!("{} - {} = {}", a, b, d);
    let e = a * b;
    println!("{} * {} = {}", a, b, e);
    let f = a as f32 / b as f32;
    println!("{} / {} = {}", a, b, f);
    let g = a % b;
    println!("{} % {} = {}", a, b, g);
}

#[test]
fn augmented_assignment_test() {
    let mut a = 10;
    println!("variable a value is {}", a);
    
    a += 10;
    println!("variable a value is {}", a);
    
    a -= 3;
    println!("variable a value is {}", a);
    
    a *= 2;
    println!("variable a value is {}", a);
    
    a /= 10;
    println!("variable a value is {}", a);
    
    a %= 3;
    println!("variable a value is {}", a);
}