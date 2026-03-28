fn main(){
    let myname=String::from("shubham");
    takemyname(&myname);
}

fn takemyname(data:&String){
    println!("yes here it is:{}", data);
}

// there are some rules in borrwoing

// 1.you can have many immmutable refrences 
// 2.you can not borrow mut more than once
// 3.cannot mix immut and mut and the same time
// 4.Scope matters (important ⚡) can use mut borrow after transfer ownership