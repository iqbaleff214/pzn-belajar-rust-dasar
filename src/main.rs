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

#[test]
fn boolean_test() {
    let a = true;
    let b: bool = false;
    
    println!("{} is true.", a);
    println!("{} is false.", b);
}

#[test]
fn comparison_test() {
    let a = 10;
    let b = 20;
    
    let result:bool = a > b;
    println!("{} > {} is {}", a, b, result);
}

#[test]
fn logical_test() {
    let a = true;
    let b = false;
    
    println!("{} and {} is {}", a, b, a && b);
    println!("{} or {} is {}", a, b, a || b);
    println!("not {} is {}", a, !a);
}

#[test]
fn character_test() {
    let a = 'a';
    let b: char = 'b';
    
    println!("{} is a char.", a);
    println!("{} is a char.", b);
}

#[test]
fn tuple_test() {
    let data = (10, 10.5, 'a', true);
    println!("{:?}", data);
    
    let data: (i32, f64, char, bool) = (10, 10.5, 'a', true);
    println!("{:?}", data);
    println!("{:?}", data.3);
    
    let (a, _, c, d) = data;
    println!("{} {} {}", a, c, d);
    
    let mut data = (10, 10.5, 'a', true);
    println!("{:?}", data);
    data.3 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
    let result: () = unit();
    println!("{:?}", result);
}

#[test]
fn array_test() {
    let data: [i32; 5] = [10, 20, 30, 40, 50];
    println!("{:?}", data);
    
    println!("{:?}", data[0]);
    println!("{:?}", data[1]);
    println!("{:?}", data[2]);
    println!("{:?}", data[3]);
    println!("{:?}", data[4]);
    
    let mut data = [10, 20, 30, 40, 50];
    println!("{:?}", data);
    data[0] = 100;
    println!("{:?}", data);
    
    println!("Size of array is {}", data.len());
}

#[test]
fn two_dimensional_array_test() {
    let data: [[i32; 5]; 3] = [
        [10, 20, 30, 40, 50], 
        [100, 200, 300, 400, 500], 
        [1000, 2000, 3000, 4000, 5000]
    ];
    
    println!("{:?}", data);
    println!("{:?}", data[0][0]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant_test() {
    const MINIMUM: i32 = 10;
    println!("Maximum value is {}.", MAXIMUM);
    println!("Minimum value is {}.", MINIMUM);
}

#[test]
fn variable_scope_test() {
    println!("global: maximum is {}", MAXIMUM);
    
    let a = 10;
    println!("outer: a is {}", a);
    {
        let a = 20;
        println!("scoped: a is {}", a);
        
        let b = 30;
        println!("scoped: b is {}", b);
    }
    
    // println!("b is {}", b); error 
}