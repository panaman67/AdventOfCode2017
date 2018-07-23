use std::fs::File;
use std::io::*;

fn main()
{
    part1();
    part2();
}

fn part1()
{
	let mut input_file = File::open("input_day1.txt").expect("No file found");

    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Problems with reading string.");
        
    let mut nums : Vec<u32> = Vec::new();

    for r in 0..input.len() - 1
    {
        nums.push(input.chars().nth(r).unwrap().to_digit(10).unwrap());
    }
    
    let mut sum : u32 = 0;
    for x in 0..nums.len() - 2
    {
        if nums[x] == nums[x + 1]
        {
            sum = sum + nums[x];
        }
    }
	
    if nums[nums.len() - 1] == nums[0]
    {
        sum = sum + nums[0];
    }
    println!("Part 1: {}", sum);
}

fn part2()
{
    let mut input_file = File::open("input_day1.txt").expect("Open file error");

    let mut input = String::new();

    input_file.read_to_string(&mut input).expect("Reading file error");

    let mut nums : Vec<u32> = Vec::new();

    for x in 0..input.len() - 1
    {
        nums.push(input.chars().nth(x).unwrap().to_digit(10).unwrap());
    }

    let mut sum : u32 = 0;
    for y in 0..(nums.len() / 2) - 1
    {
        if nums[y] == nums[y + (nums.len() / 2)]
        {
            sum += nums[y];
        }
    }
    println!("Part 2: {}", sum);
}

