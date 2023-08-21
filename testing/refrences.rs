fn main() {
    let mut x: i32 = 10; //Normal Integer
    let ref_x: &mut i32 = &mut x; //A mutable refrence to a mutable integer
    let mut y: i32 = *ref_x;
    y += 100;
    *ref_x = 100;
    println!("Refrence to an Integer");
    println!("x: {y}");
    println!("x: {x}");
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    //reading
    let val: i32 = s[1];
    println!("s: {s:?}");
    println!("val: {val:?}");
    // Strings vs str
    let s1: &str = "World";
    let mut s2: String = String::from("Hello ");
    s2.push_str(s1);
    let s3: &str = &s2[6..];
    println!("s3 : {s3}");
}
