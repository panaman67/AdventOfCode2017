use std::io::{BufRead, BufReader};
//use std::str::*;
use std::fs::File;

fn main()
{
    part1();
    part2();
}

fn part1()
{
    let input_file = File::open("input_day4.txt").expect("Error opening");
    let buf = BufReader::new(input_file);

    let mut sum : i16 = 0;
    'outer: for line in buf.lines()
    {
        let line = line.unwrap();
        let input : Vec<&str> = line.split(char::is_whitespace).collect();
        
        for x in 0..input.iter().len()
        {
            for y in 0..input.len()
            {
                if x != y && input[x] == input[y]
                {
                    continue 'outer;
                }
            }
        }       
        sum += 1;
    }
    println!{"Part 1: {}", sum};
}

fn part2()
{
    let input_file = File::open("input_day4.txt").expect("Error opening file");
    let buf = BufReader::new(input_file);
    

    let mut sum : i16 = 0;
    'outer:
    for line in buf.lines()
    {
        let line = line.unwrap();
        let input : Vec<&str> = line.split(char::is_whitespace).collect();
        
        //  ["mary", "little", "lamb", "ramy"]
        let input : Vec<String> = input.iter().map(|x| String::from(*x)).collect();

        let mut words : Vec<Vec<u8>> = Vec::new();
        for x in input.iter()
        {
            words.push((*x).as_bytes().to_vec());
        }
        
        for x in words.iter_mut()
        {
            (*x).sort();
        }
        /*
        for y in words.iter()
        {
            if words.contains(y)
            {
                continue 'outer;
            }
        }
        */
        //let new : Vec<Vec<u8>> = words.dedup().clone();
        let length = words.len();
        words.sort();
        words.dedup();
        if length == words.len()
        {
            sum += 1;
        }
    }
    println!("Part 2: {}", sum);
}









