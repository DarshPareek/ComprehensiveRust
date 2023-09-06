fn main() {
    let r: [usize; 100] = core::array::from_fn(|i| i + 1);
    println!("{r:?}");
}
