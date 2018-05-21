use std::io::prelude::*;
use std::fs::File;

fn main()
{
    let mut input_file = File::open("input_day9.txt").expect("cant find file");
    let mut input = String::new();

    input_file.read_to_string(&mut input).expect("Could not read into string");

    //println!("{}", input);

    //let mut test = input.replace("<>", "");
    
    //println!("{}", test);
    
    loop 
    {
        break;
    }
}
