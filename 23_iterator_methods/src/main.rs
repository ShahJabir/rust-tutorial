fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let double_vec: Vec<i32> = vec.iter().map(|x|x*2).collect();
    println!("{:?}",double_vec);
    let double_vec: Vec<i32> = vec.into_iter().map(|x| x*2).collect();
    println!("{:?}", double_vec);
}
