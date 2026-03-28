fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let a = 23;
    let b = 45;

    let result = swap(a, b);

    println!("{:?}", result); // (45, 23)
}