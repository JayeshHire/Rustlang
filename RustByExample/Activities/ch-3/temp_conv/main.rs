
fn main(){
    let fah = 35.0;
    let cel = fah_2_cel(fah);
    println!("fah value {} in celsius is {}",fah, cel);
    let cel = 30.0;
    let fah = cel_2_fah(cel);
    println!("celsius value {} in fahrenheit is {}", cel, fah);
}   

//fahrenheit to celsius converter
fn fah_2_cel(fah: f64) -> f64 {
    let cel : f64 = (fah - 32.0)*<i32 as Into<f64>>::into(5/9);
    cel
    }

//celsius to fahrenheit converter
fn cel_2_fah(cel : f64) -> f64 {
    let fah : f64 = cel*<i32 as Into<f64>>::into(5/9) + 32.0;
    fah
    }

