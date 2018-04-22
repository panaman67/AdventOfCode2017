use std::fs::File;
use std::io::{BufRead, BufReader};

fn main()
{
    part1();
    part2();
}

fn part1()
{
    let input_file = File::open("input_day5.txt").expect("Error opening file");
    let buf = BufReader::new(input_file);
    
    let mut input : Vec<i32> = Vec::new();
    for line in buf.lines()
    {
        let line = line.unwrap();
        input.push(line.trim().parse::<i32>().unwrap());
    }
    
    let mut steps : i32 = 0;
    let mut cur_value : i32;
    let mut pos : i32 = 0;

    while pos < input.len() as i32
    {
        cur_value = input[pos as usize];
        input[pos as usize] += 1;
        pos += cur_value;
        steps += 1;
    }
    println!("Part 1: {}", steps);
}

fn part2()
{
    let input_file = File::open("input_day5.txt").expect("Error opening file");
    let buf = BufReader::new(input_file);
    
    let mut input : Vec<i32> = Vec::new();
    for line in buf.lines()
    {
        let line = line.unwrap();
        input.push(line.trim().parse::<i32>().unwrap());
    }
    
    let mut steps : i32 = 0;
    let mut cur_value : i32;
    let mut pos : i32 = 0;

    while pos < input.len() as i32
    {
        cur_value = input[pos as usize];
        if cur_value >= 3
        {
            input[pos as usize] -= 1;
        }
        else
        {
            input[pos as usize] += 1;
        }
        pos += cur_value;
        steps += 1;
    }
    println!("Part 2: {}", steps);
}



