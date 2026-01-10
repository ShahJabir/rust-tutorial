fn main() {
    let result: &i32;
    let x = 20;
    let y = 10;
    result = larger(&x, &y);
    println!("{}", result);
}

fn larger<'a>(m: &'a i32, n: &'a i32) -> &'a i32 {
    if m > n { m } else { n }
}
