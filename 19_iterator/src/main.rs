fn main() {
    let vec:Vec<i32> =vec![1,2,3];
    let vec2 = vec![5,6,7];
    for item in vec.iter() {
      println!("reference of vec {}",item)
    }

    for item in vec.into_iter() {
      println!("move ownership of vec {}",item)
    }
    // This line will throw error
    // println!("{:?}",vec)
    for item in vec2 {
      println!("In default loop use into_iter to transfer ownership of heap memory vector: {}",item);
    }
    // This line will throw error
    // println!("{:?}",vec2)
}
