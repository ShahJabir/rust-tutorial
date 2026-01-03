fn main() {
  let mut arr1:[&str;3] = ["Hello","World","Rust"];
  write_arr(arr1);
  println!("outside write_arr fn: {:?}",arr1);
  ref_write_arr(&mut arr1);
  println!("outside ref_write_arr fn: {:?}",arr1);
}

fn write_arr(mut arr:[&str;3]){
  arr[0] = "Fellow";
  println!("inside write_arr fn: {:?}",arr)
}

fn ref_write_arr(arr:&mut[&str;3]) {
  arr[0] = "Fellow";
  println!("inside ref_write_arr fn: {:?}",arr)
}
