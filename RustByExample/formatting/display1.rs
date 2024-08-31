use std::fmt ;

struct Furniture {
    weight : u32 ,
    volume : u32
} 

impl fmt::Display for Furniture {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "weight: {}\nvolume: {}", self.weight, self.volume)
    }
}

fn main () 
{
    let table = Furniture {weight : 40, volume: 10};
    println!("Furniture record: {}", table);
}