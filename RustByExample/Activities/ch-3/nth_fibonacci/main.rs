fn main(){
    loop {
    let mut user_input = String::new();
    println!("Enter the position of the fibonacci number");
    let _b = std::io::stdin().read_line(&mut user_input).unwrap();
    if user_input == String::from("q") {
        break;
        }
    
    let n : i32 = match user_input.trim().parse() {
        Ok(number) => number, 
        Err(_e) => break,
    };
    let fib_no = fib(n);
    println!("The fibonacci number at {} position is {}",n, fib_no);
    }
}

fn fib(n : i32) -> i32 {
    let mut first = 1;
    let mut second  = 2;
    if n == 1 {
        return first;
    } else if n ==2 {
       return  second;
    } else {
        for _i in 2..n {
            let temp = first + second;
            first = second;
            second = temp;
        }
        return second;
    }
     
}
