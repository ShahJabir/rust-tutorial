struct Workshop {
  title: String,
  instructor: String,
  duration: u8
}

struct Seminar {
  title: String,
  speaker: String,
  location: String
}

trait Course {
  fn get_overview(&self) -> String;
}

impl Workshop {
    fn new(title: String, instructor: String, duration: u8) -> Self{
      Self { title, instructor, duration }
    }
}

impl Course for Workshop {
  fn get_overview(&self) -> String {
    format!("Workshop title:{} Workshop instructor:{} Workshop duration:{}", self.title, self.instructor, self.duration)
  }
}

impl Seminar {
  fn new(title:String, speaker:String, location:String) -> Self{
    Self { title, speaker, location }
  }
}

impl Course for Seminar {
  fn get_overview(&self) -> String {
    format!("Seminar title:{} Seminar speaker:{} Seminar location:{}", self.title, self.speaker, self.location)
  }
}

fn print_course_overview<T:Course>(course: T){
  println!("{}", course.get_overview())
}

fn main() {
    let devops_workshop = Workshop::new("DevOps".to_owned(), "Shah Jabir Taqi".to_owned(), 100);
    let web_seminar = Seminar::new("Web Development".to_owned(), "Shah Jabir Taqi".to_owned(), "Rajshahi High Tech Park".to_owned());
    print_course_overview(devops_workshop);
    print_course_overview(web_seminar);
}
