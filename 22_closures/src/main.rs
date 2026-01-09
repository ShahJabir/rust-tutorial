fn main() {
    let add = |x:u8| x+1;
    println!("{}", add(5));
    // ----------------------------------------------------------------
    let mut counter: u8 = 0;
    let mut count = || {
      counter += 1;
      println!("{}",counter);
    };
    count();
    count();
    count();

    // ---------------------------------------------------------------

    let x:String = "Hello, World!".to_owned();
    let closure_fn_once = || x;
    println!("{}",closure_fn_once());
}
