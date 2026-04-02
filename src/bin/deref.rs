use std::ops::Deref;

// Step 1: Create Wrapper<T>
struct Wrapper<T>(T);

// Step 2: Implement Deref
impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Step 3: Function that expects &str
fn double_len(s: &str) -> usize {
    s.len() * 2
}

// Step 4: Driver code
fn main() {
    let w = Wrapper(String::from("hello"));

    // Passing &Wrapper<String>
    // Rust will do:
    // &Wrapper<String> → &String → &str
    let result = double_len(&w);

    println!("Double length: {}", result);
}