fn main() {
    println!("{:?}", second_word_to_upper("foo bar"));
    println!("{:?}", second_word_to_upper_mod("foo bar"));
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
    let t = match std::env::args().next().as_deref() {
        Some("cat") => "Will do cat things",
        Some("ls") => "Will ls some files",
        Some("mv") => "Let's move some files",
        Some("rm") => "Uh, dangerous!",
        None => "Hmm, no program name?",
        _ => "Unknown program name!",
    };
    println!("{}", t);
    inspect(&[0, -2, 3]);
    inspect(&[0, -2, 3, 4]);
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}
fn second_word_to_upper_mod(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    if let (Some(_), Some(item)) = (it.next(), it.next()) {
        return Some(item.to_uppercase());
    } else {
        return None;
    };
}
#[rustfmt::skip]
    fn inspect(slice: &[i32]) {
        println!("Tell me about {slice:?}");
        match slice {
            &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
            &[1, ..]   => println!("First is 1 and the rest were ignored"),
            &[..,_] => println!("using _ to represent last one"),
            &[.., b] => println!("Tail is {b}"),
           // &[a @ .., b] => println!("{:?}..{}", a, b), gives a wierd error i will try to find soln
            _          => println!("All element:s were ignored"),
        }
    }
