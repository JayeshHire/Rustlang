use std::fmt ;

#[derive(Debug)]
struct Color {
    red : u8,
    green : u8,
    blue : u8,
}

impl fmt::Display for Color {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rgb = (self.red as u32)*65536 + (self.green as u32)*256 + (self.blue as u32);
        write!(f, "RGB ({}, {}, {}) 0x{:0X}",self.red, self.green, self.blue, rgb)
    }
}

fn main(){
   /* let pune = City {
        name : "Pune",
        lon : 43.85764,
        lat : 65.9873,
    }  ;
     println!("{}", pune);
    */
    let mycolor = Color {
        red : 43,
        green : 203,
        blue : 123
    } ;
   
    println!("{}", mycolor);
}
