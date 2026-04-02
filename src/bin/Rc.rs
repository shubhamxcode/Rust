use std::rc::Rc;

// fn main(){
//     let data1=Rc::new(String::from("shubham"));
//     let clone1=Rc::clone(&data1);
//     let clone2 = Rc::clone(&data1);

//     println!("count:{}",Rc::strong_count(&data1));
//     println!("count:{}",Rc::strong_count(&clone1));
//     println!("count:{}",Rc::strong_count(&clone2));
    
// }



fn count_owner(n:usize)->usize{
    let shared=Rc::new(String::from("shubham"));
    let mut clone=Vec::new();

     for _ in 0..n {
        clone.push(Rc::clone(&shared));
    };
        Rc::strong_count(&shared)
}

fn main() {
    let count = count_owner(5);
    println!("Total owners: {}", count); // 6
}