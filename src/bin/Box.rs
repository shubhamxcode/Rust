struct LL {
    value: i32,
    next: Option<Box<LL>>,
}

fn main() {
    let l = LL {
        value: 1,
        next: Some(Box::new(LL {
            value: 2,
             next:Some(Box::new(LL{
                value:22,
                next:None,
             }))
        })),
    };

    println!("Linked list created!");
}