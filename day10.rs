use std::io::{BufReader, BufRead};
use std::fs::File;

fn main()
{
    let input_file = File::open("input_day10.txt").expect("Error opening file");

    let mut inputstr : String = String::new();

    let mut buf = BufReader::new(input_file);

    let mut skip_num : u8 = 0;
    let mut list : Vec<u8> = Vec::new();
    for x in 0..256
    {
        list.push(x as u8);
    }

    buf.read_line(&mut inputstr).expect("Error reading file");
    let inputstr = inputstr.trim();

    let lengths_array : Vec<u8> = inputstr.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    
    let mut pos = 0;

}
