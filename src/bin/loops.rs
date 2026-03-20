fn main() {
    let name = String::from("Hi guys my name is shubham varshney!");
    let first_word = get_first_word(name);
    println!("my first word is: {}", first_word);
}

fn get_first_word(name: String) -> String {
    let mut ans = String::from("");

    for ch in name.chars() {
        if ch == ' ' {
            break;
        }
        ans.push_str(ch.to_string().as_str());
    }

    ans
}