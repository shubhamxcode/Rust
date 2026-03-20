fn main (){

    let greetings=String::from("hello world!");
    let char1=greetings.chars().nth(10000);

    match char1{
        Some(c)=>println!("{}",c),
        None=>println!("no character is there"),
    }
}