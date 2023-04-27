use std::io ;


fn main() {

	println!("Temperature converter");
'flow : loop{
	println!("1. Fahrenheit to celsius \n2.Celsius to Fahrenheit \n3. To quit");
	println!("Enter your choice :");
	let mut choice = String::new();
	let stdin = io::stdin();
		stdin.read_line(&mut choice)
		.expect("Failed to read line");
	let choice : i32 = match choice.trim().parse() {
		Ok(num) => num ,
		Err(_) => continue ,
		};


if choice == 1 {
	let mut fh = String::new();
	println!("Enter the value in Fahrenheit :");
	stdin.read_line(&mut fh)
.expect("Failed to read line");
		//taking input from the user
	//parsing it into integer type
let fh : f64 = match fh.trim().parse() {
	Ok(num) => num , 
	Err(_) => continue ,
	};

let cel : f64 = fh_to_cel(fh) ;
println!("Temperature in Celsius is {cel}");
			

} else if choice == 2 {
let mut cel = String::new();
	println!("Enter the Temperature in Celsius :");
	stdin.read_line(&mut cel)
.expect("Failed to read line");
		//taking input from the user
	//parsing it into integer type
let cel : f64 = match cel.trim().parse() {
	Ok(num) => num , 
	Err(_) => continue ,
	};

let fh : f64 = cel_to_fh(cel) ;
println!("Temperature in Fahrenheit is {fh}");

	
} else if choice == 3 {
	println!(" Exiting ...");
	break 'flow ;
} else {
	 println!("Enter valid choice");
}
	}


}


fn fh_to_cel(fh : f64) -> f64{
(fh -32.0)*5.0/9.0 
}

fn cel_to_fh(cel : f64) -> f64{
cel*9.0/5.0 + 32.0
}
