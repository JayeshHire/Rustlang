fn main{

    let days : [String; 9] = ["first", "second","third","fourth", "fifth", 
    "sixth", "seventh", "eighth", "ninth"];
    let numbers : [String;8 ] = ["two", "three", "four", "five", "six", "seven",
    "eight", "nine"];
    let sent  = String::from("my true love sent to me");
    let gifts : [String; 9] = ["partridge", "turtle doves", "french hens", 
    "calling birds", "golden rings", "geese a-laying", "swans a-swimming", 
    "maids a-milking","ladies dancing"];
    let mut first_line = String::from(" ");
    let mut song = String::from(" ");

    for i in 0..9 {
        first_line = format!("On the {} day of Christmas, {}", days[i], sent);
        if i == 0 {
            song = format!("A {} in a pear tree",gifts[0]);
        } else if i == 1 {
            //write a capitalize function
            let b : u8 = numbers[0].to_uppercase().as_bytes()[0];
            let firstc : char = b as char ;
            let 
            song = format!(" {} {}  and {} in a peer tree",firstc, 
                           gifts[1],gifts[0]);
        } else {
            let b : u8 = numbers[i-1].to_uppercase().as_bytes()[0]


}
fn capitalize( word : &str) ->String {
    let firstc = word.to_uppercase()
    .as_str()
    .chars().nth(0).unwrap();
    let mut capital_word = String::new();
    capital_word = format!("{}",firstc);
    for i in 1..word.len() {
        let letter = word.chars().nth(i).unwrap();
        capital_word = format!("{}{}",capital_word, letter);
    }
    format!("{}", capital_word)
}
