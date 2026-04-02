

// A traits define the functionality of a particular type that can we share with other types. we can use traits to shared behaviour in abstract way 

pub trait Summary {
    fn summarize(&self) -> String{
        return String::from("rust is very hard bro");
        // default function 
    }
}

struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("my name is: {} and my age is: {}", self.name, self.age);
    }
}

fn main() {
    let data = User {
        name: String::from("shubham"),
        age: 20,
    };

    println!("{}", data.summarize());
}