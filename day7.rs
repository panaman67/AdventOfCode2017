use std::fs::File;
use std::io::{BufReader, BufRead};

fn main()
{
    let input_file = File::open("input_day7.txt").expect("Error on file");

    let mut buf = BufReader::new(&input_file);
    
    let mut map : Vec<String>;
    let mut program : String = "".to_owned();
    let mut num : u16 = u16::max_value();
    for line in buf.lines()
    {
        let line = line.unwrap();
        let line = line.trim();

        map = line.split(" ").map(|x| x.to_owned()).collect();
        map[1] = map[1].replace("(", "");
        map[1] = map[1].replace(")", "");
        
        let temp = map[1].parse::<u16>().unwrap();
        if temp < num
        {
            num = temp;
            program = map[0].clone();
        }
    }
    println!("{}", program);
}
