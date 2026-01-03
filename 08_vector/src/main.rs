use std::vec;

fn main() {
    // let mut v:Vec<i32> = Vec::new();
    let mut v = Vec::<i32>::new();
    let v1: Vec<i32> = vec![1,2,3,4,5];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v Vector: {:?}",v);
    println!("v1 Vector: {:?}",v1);
    println!("______________________");
    read_vector(v.clone());
    println!("After read_vector fn: {:?}", v);
    println!("______________________");
    ref_read_vector(&v1);
    println!("After ref_read_vector fn: {:?}",v1);
}

fn read_vector(v:Vec<i32>) {
  println!("Inside read_vector fn: {:?}", v)
}

fn ref_read_vector(v:&Vec<i32>) {
  println!("Inside ref_read_vector fn: {:?}",v)
}
