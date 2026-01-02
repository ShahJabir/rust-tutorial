fn main() {
  // By referencing you can allow only to read multiple variable but if you write or mut 
    let mut s1:String = String::from("Hello");
    let mut x = 5;
    x+=1;
    let w1: &mut String = &mut s1;
    (*w1).push_str(" World");
    println!("r1: {}", w1);
    let w2: &mut String = &mut s1;
    w2.push_str(" Code!");
    println!("r2: {}", w2);
    // Next line is throwing error to use w1 after declear w2
    // println!("w1: {}, w2: {}", w1, w2);
    let y = &mut x;
    *y=*y+1;
    println!("x = {}",y);
    println!("x address is = {:p}",&y)
}
