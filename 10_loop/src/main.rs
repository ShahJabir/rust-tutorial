fn main() {
    let mut i = 0;
    while i < 5 {
        println!("Hello");
        i += 1;
    }
    for _ in 0..5 {
        println!("World");
    }
    (0..5).for_each(|_| println!("Rust"));
}
