fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
    let mut x = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}
