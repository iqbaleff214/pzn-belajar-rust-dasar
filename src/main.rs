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