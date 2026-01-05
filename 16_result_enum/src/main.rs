fn divide( x:i32, y:i32 ) -> Result<i32,String> {
  if y == 0 {
    return Err("You can't divide any number with zero".to_owned())
  } else {
    return Ok(x/y)
  }
}

fn main() {
    let result = divide(10, 0);
    println!("{:?}",result);
}
