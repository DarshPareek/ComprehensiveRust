fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;
    //let z = bool::from(x);
    //let w = bool::from(y); Doesnt work bcuz cant convert i16 to bool
    //println!("{z} * {w} ");
    println!("{x} * {y} = {}", multiply(x.into(), y));
}
