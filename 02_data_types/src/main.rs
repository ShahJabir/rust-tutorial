fn main() {
    let mut num:u16 = 65535;
    println!("This is stored in num is {}",num);

    num = 200;
    println!("This is stored in num is {}", num);

    // This is a unmutable string using &str
    let name:&str = "Shah Jabir Taqi";
    println!("Name is {}", name);

    // This is a mutable string using String
    let mut sub_name = String::from("Taqi Jabir");
    println!("Sub name before {}", sub_name);
    sub_name.push_str(" Hacker");
    println!("Sub name after {}", sub_name);

    // Tuple Data Type
    let emp_info:(&str, u8) = ("John Doe", 30);
    let (emp_name, emp_age) = emp_info;
    println!("Employ name: {} and Employ age: {}", emp_name, emp_age)
}
