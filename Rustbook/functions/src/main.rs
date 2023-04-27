fn main() {
    println!("Hello, world!");
    print_labeled_measurement(32,'m');
}


fn print_labeled_measurement(value:i32 , unit_measurement:char){
    println!("The measurement is: {} {} ", value, unit_measurement);
}
