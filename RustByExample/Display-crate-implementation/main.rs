use std::fmt ;

//struct in rust
#[derive(Debug)]
struct FamilyMembers {
    total : u32,
    children : u32,
    max_age : u32,
    min_age : u32,
}



#[derive(Debug)]
struct MemberNames ( String, String, String, String);

//implementing Display crate for MemberNames struct
impl fmt::Display for MemberNames {
/*
this is the standard signature for implementing the Display crate on a struct 
do not make changes into it
*/
    fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "first member: {} 
        second member :{} 
        third member :{} 
        fourth member : {}"
        , self.0
        , self.1
        , self.2
        , self.3)
    }
}

//implementing Display crate for FamilyMembers struct
impl fmt::Display for FamilyMembers {
    /*
this is the standard signature for implementing the Display crate on a struct 
do not make changes into it
*/
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, " total family members : {} \nNo. of children : {} \nMaximum age of family member : {} \nMinimum age of family member : {}"
        ,self.total, self.children, self.max_age, self.min_age)
    }
}

fn main(){

    //creating an instance for FamilyMembers struct using function create_family
    let mut family1 = create_family(4, 2, 45, 12);
    
    /*
    copying one struct into another
    Here Except the values we specify 
    other all values will be copied
    from family1 struct
    */
    let family2 = FamilyMembers {
        total : 7,
        ..family1
    };
    
    //creating an instance of MemberNames struct
    let family_names = MemberNames(
        return_string("Soham"),
        return_string("Jay"),
        return_string("Jayesh"),
        return_string("Dadu")
    );
    
    println!("{}", family_names);
 //   println!("{:#?}", family_names);
    family1.total = 8;
    println!("{}",family1 );
    println!("{}",family2.max_age);
    
    
}

//function for creating an instance of the family struct
fn create_family(total : u32, children : u32, max_age: u32, min_age : u32) -> FamilyMembers {
    let family = FamilyMembers {
        total,
        children,
        max_age,
        min_age,
    };
     family
}

//function to return string type which will be used by MemberNames struct
fn return_string(name : &str) -> String {
    String::from(name)
}