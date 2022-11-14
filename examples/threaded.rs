fn main() {
    let mut counter = Counter::new();
    let mut handles = Vec::new();
    for _ in 0..100 {
        let handle = std::thread::spawn(|| counter.inc());
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Count: {}", counter.n)
}

struct Counter {
    n: usize,
}

impl Counter {
    fn new() -> Self {
        Self { n: 0 }
    }

    fn inc(&mut self) {
        self.n += 1;
    }
}
