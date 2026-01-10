fn main() {
    // Map
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let double_vec: Vec<i32> = vec.iter().map(|x|x*2).collect();
    println!("{:?}",double_vec);
    let double_vec: Vec<i32> = vec.into_iter().map(|x| x*2).collect();
    println!("{:?}", double_vec);

    // Filter
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_vec:Vec<&i32> = vec.iter().filter(|x| *x%2 == 0).collect();
    println!("{:?}",even_vec);
    let even_vec:Vec<i32> = vec.into_iter().filter(|x| x%2 == 0).collect();
    println!("{:?}",even_vec);

    // reduce
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match vec.into_iter().reduce(|accumulator, item| accumulator+item) {
        Some(sum) => println!("The sum of vector elements is :{}", sum),
        None => println!("None"),
    }
}
