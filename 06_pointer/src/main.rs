fn main() {
    let x = 5;
    let y: *const i32 = &x as *const i32;  // Raw pointer (const)
    
    // Must use unsafe to dereference
    unsafe {
        println!("y: {}", *y);
    }
    
    let mut a = 10;
    let b: *mut i32 = &mut a as *mut i32;  // Raw pointer (mut)
    
    unsafe {
        *b += 5;
        println!("b: {}", *b);
        example()
    }
}

fn example() {
    let r: *const i32;
    {
        let x = 5;
        r = &x as *const i32;
    }
    
    // Overwrite stack with different values
    let _ = heavy_computation(100);
    let _ = heavy_computation(200);
    let _ = heavy_computation(300);
    
    unsafe {
        println!("r: {}", *r);  // Likely garbage now
    }
}

fn heavy_computation(val: i32) -> i32 {
    let temp1 = val * 2;
    let temp2 = temp1 + 10;
    let temp3:i32 = temp2 - 5;
    println!("{}",temp3);
    return temp3;
}
