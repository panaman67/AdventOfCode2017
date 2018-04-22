use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main()
{
    part1();
    part2();
}

fn part1()
{
    let f = File::open("input_day2.txt").expect("");
    let f = BufReader::new(f);
    
    let mut max;
    let mut min;
    let mut sum : i32 = 0;

    for line in f.lines()
    {
        max = std::i32::MIN;
        min = std::i32::MAX;

        let line = line.unwrap();
        let line : Vec<&str> = line.split('\t').collect();

        let mut row : Vec<i32> = Vec::new();
        for x in 0..line.iter().len()
        {
            row.push(String::from(line[x]).parse::<i32>().unwrap());
        }

        for y in row.iter()
        {
            if *y > max
            {
                max = *y;
            }
            else if *y < min
            {
                min = *y;
            }
        }
        sum += max - min;
    }
    println!("Part 1: {}", sum);
}

fn part2()
{
    let input_file = File::open("input_day2.txt").expect("Error opening file");
    let buf = BufReader::new(input_file);
    
    let mut sum : i32 = 0;
    for line in buf.lines()
    {
        let line = line.unwrap();
        let input : Vec<&str> = line.split('\t').collect();
        
        let mut nums : Vec<i32> = Vec::new();
        for x in 0..input.iter().len()
        {
            nums.push(String::from(input[x]).parse::<i32>().unwrap());
        }
        
        //println!("{:?}", nums);

        for x in 0..nums.iter().len()
        {
            for y in 0..nums.iter().len()
            {
                if x != y && nums[x] % nums[y] == 0
                {
                    sum += nums[x] / nums[y];
                }
            }
        }
    }
    println!("Part 2: {}", sum);
}














