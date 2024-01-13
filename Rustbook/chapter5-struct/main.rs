use std::{fmt, f64} ;

#[derive(Debug)]
struct Rectangle {
    length : u32,
    width : u32
}

struct Family {
    no_of_members : u32,
    eldest_member_age : u32,
    youngest_member_age : u32,
    adress : String,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "length : {}\nwidth : {}",self.length, self.width)
    }
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.length*self.width
    }
    
    fn square(side : u32) -> Self {
        Self {
            length : side,
            width : side
        }
    }
}

#[derive(Debug)]
struct Point (i32, i32, i32);
struct LineVector<'s>( &'s Point,&'s Point);

impl fmt::Display for Point {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x, y, z) : ({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        let num = ((self.0 - p.0)*(self.0 - p.0) +
            (self.1 -p.1)*(self.1 - p.1) +
            (self.2 - p.2)*(self.2 - p.2)) as f64;
        f64::sqrt(num)
    }
    fn p2p_distance(p :&Point, q: &Point) -> f64 {
        let num = ((p.0 - q.0)*(p.0 - q.0) +
            (p.1 - q.1)*(p.1 - q.1) +
            (p.2- q.2)*(p.2 - q.2) ) as f64;
        f64::sqrt(num)
    }
}
fn main(){
    let family1 = Family {
        no_of_members : 4,
        eldest_member_age : 40,
        youngest_member_age : 18,
        adress : String::from("A/p Waygaon, tal-Malegaon, Dist- Nashik")
    };
    
    let p1 = Point( 23,12, 9);
    let p2 = Point(9, 8, 1);
    let l1 = LineVector( &p1,  &p2);
   
    println!("distance : {}",p1.distance(&p2));
    println!("distance : {}", Point::p2p_distance(&p1, &p2));
    println!("end-point 1 : {:?}", l1.0);
    println!("end-point 2 : {:?}", l1.1);

    let rect1 = Rectangle {
        length : 32,
        width : 12
    };
    let sq1 = Rectangle::square(32);
    dbg!(&sq1);
   println!(" {:#?}", sq1);
}


