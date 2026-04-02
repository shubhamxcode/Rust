fn main() {
    let list = vec![1,2,3,4,5,6,7,8];
    let list2 = vec![1.0,2.0,3.0,4.0,5.0];

    let l = largest(&list);
    let l2 = largest(&list2);

    println!("The largest number is :{}", l);
    println!("The largest number is :{}", l2);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) ->&T {
    let mut result = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}