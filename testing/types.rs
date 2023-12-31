fn main() {
    println!("The Integers");
    let i1: i8 = 52;
    let i2: u8 = 52;
    let mut f1: f32 = 744.23;
    f1 += 999.15;
    println!("One 8 bit signed int {i1}");
    println!("One 8 bit unsigned int {i2}");
    println!("One 32 bit mutable float {f1}");
    println!("The Strings and Chars");
    let s1: &str = "Hello";
    let mut s2: &str = "World";
    //s2[3] = "R";
    let c1: char = '🌎';
    println!("A Sting saying Hello World but the first word is not mutable : {s1} {s2}");
    println!("A char {c1}");
    println!("The Booleans");
    let b1: bool = true;
    let mut b2: bool = true;
    println!("A b1 immutable boolean {b1}");
    println!("A b2 mutable boolean {b2}");
    b2 = false;
    println!("A b2 after mutation {b2}");
    println!("The Arrays and Tuples");
    let mut a1: [i32; 10] = [22; 10];
    let mut t1: (i32, bool, char, &str) = (418, true, '@', "i am a teapot");
    println!("An array {:#?} also has pretty print ", a1);
    a1[6] = 2222;
    println!("An array {:?}", a1);
    println!("A tuple {:#?}", t1);
    t1.1 = false;
    t1.3 = "i am not a teapot";
    println!("same tuple {:#?}", t1);
}
