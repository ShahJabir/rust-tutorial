// Define the Counter struct
struct Counter {
    count: u32,
}


// Implement methods for Counter
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


// Implement the Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32;


    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}


// Main function to use the iterator
fn main() {
    let counter = Counter::new();


    for number in counter {
        println!("{}", number);
    }
}


