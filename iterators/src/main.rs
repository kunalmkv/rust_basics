struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= 5 {
            self.count = self.count + 1;
            return Some(self.count);
        } else {
            return None;
        }
    }
}

fn main() {
    let count: Counter = Counter::new();
    for val in count {
    println!("Count: {:?}", val);
    }
}