use std::io;

fn main() {
    let a = [1,2,3,4,5];
    loop{
    let mut i = String::new();

    println!("Enter the index of the array ");
    io::stdin()
        .read_line(&mut i)
        .expect("Failed to read line");

    let i : usize = match i.trim().parse() {
        Ok(num) => num ,
        Err(_e) => {
            println!("Enter a valid number");
            continue;
        } ,
    };
    if i>=5 {
        println!("Index of the array is out of bound");
        continue;
    }
    let element = a[i];
    println!("the value of the element at index {i} is {element}");
    }
}

