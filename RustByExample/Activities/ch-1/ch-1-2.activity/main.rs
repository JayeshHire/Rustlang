use std::fmt ;


#[derive(Debug)]
struct Complex {
    real : f64,
    img : f64,
}

impl fmt::Display for Complex {
    fn fmt (&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

fn main(){
    let comp1 = Complex {
        real : 3.3,
        img : 7.2,
    } ;
    println!("Display : {}", comp1);
    println!("Debug : {:?}", comp1);
}
