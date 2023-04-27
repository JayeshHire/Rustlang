use std::io ;

fn main() {
    println!("Hello, world!");
	let stdin = io::stdin();

loop{
let mut nth = String::new();
println!("Enter the nth no for fibonacci series (press any key to exit and then enter) ");
stdin.read_line(&mut nth)
.expect("Failed to read line");



let nth : i32 = match nth.trim().parse() {
			Ok(num) => num ,
			Err(_) => {
				println!("Exiting ..");
				break; } ,
} ;
let fibo = nth_fibonacci(nth);
println!("The nth Fibonacci number is {fibo}");
}
}


fn nth_fibonacci(n : i32) -> i32 {
	let mut am = 0;
	let mut an = 1 ;
	
	for _i in 0..n {
		let  temp : i32 = an + am;
		am = an ;
		an = temp ;
		}
	am
	}


		
		
	
	
	