#[derive(Debug)]
struct Student {
  name: String,
  age: u8,
  pass: bool,
}

#[derive(Debug)]
struct Rectangle {
  length: u16,
  breadth: u16
}

impl Student {
    fn new(name:String, age:u8, pass:bool) -> Student {
      Student { name:name, age:age, pass:pass }
    }
}

impl Rectangle {
  // Initialize function
  fn new(length: u16,breadth: u16) -> Self {
    Self{length:length, breadth:breadth}
  }

  // change len function
  fn rec_area(&self) -> u16 {
  let result = self.length * self.breadth;
  return result;
  }

  fn change_value(&mut self) {
    self.length += 1;
    self.breadth -= 1;
  }
}



fn main() {
    let mut rec = Rectangle::new(100, 120);
    let stu:Student = Student::new("Ab.Rahim".to_owned(), 21, true);
    println!("{:?}",stu);
    println!("Student name: {}",stu.name);
    println!("Student age: {}",stu.age);
    println!("Student pass: {}",stu.pass);
    println!("rec1: {:?}",rec);
    let area:u16 = rec.rec_area();
    println!("rec area: {}", area);
    println!("rec before fn: {:?}",rec);
    rec.change_value();
    println!("rec after fn: {:?}",rec);
}
