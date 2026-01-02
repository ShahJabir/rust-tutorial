const GLOBAL:u8 = 10;
fn main() {
    let parent:u8 = 50;
    {
      let child:u8 = 60;
      println!("parent {{from scope}}: {}", parent);
      println!("child {{from scope}}: {}", child);
      println!("GLOBAL {{from scope}}: {}", GLOBAL);
    }
    println!("parent {{from scope}}: {}", parent);
    // This program will throw error due to this variable is not in this scope
    // println!("child {{from scope}}: {}", child); 
    println!("GLOBAL {{from scope}}: {}", GLOBAL);

    // static vs heap 
    // static
    let a:u8 = 10;
    let b:u8 = a;
    println!("a: {}", a);
    println!("b: {}", b);
    // heap
    let name = String::from("Shah Jabir Taqi");
    let sub_name = name;
    let university:String = String::from("Varendra University");
    println!("Sub Name: {}", sub_name);
    printf(university);
    // println!("{}",university)
  }

  fn printf(value:String) {
    println!("printf function {}",value);
  }
