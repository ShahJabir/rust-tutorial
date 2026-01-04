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
  fn new(length: u16,breadth: u16) -> Rectangle {
    Rectangle{length:length, breadth:breadth}
  }
}

fn change_len(rec: &mut Rectangle) {
  rec.length += 1;
  rec.breadth -= 1;
}

fn main() {
  let mut rec = Rectangle {
      length: 100,
      breadth: 120,
    };
    let rec1 = Rectangle::new(100, 200);
    let stu:Student = Student { name: "Shah Jabir".to_owned(), age: 22, pass: true };
    let stu1:Student = Student::new("Ab.Rahim".to_owned(), 21, true);
    println!("{:?}",stu1);
    println!("{:?}",stu);
    println!("Student name: {}",stu.name);
    println!("Student age: {}",stu.age);
    println!("Student pass: {}",stu.pass);
    println!("rec1: {:?}",rec1);
    println!("rec length and breadth before fn {}, {}", rec.length, rec.breadth);
    change_len(&mut rec);
    println!("rec length and breadth after fn {}, {}", rec.length, rec.breadth);
}
