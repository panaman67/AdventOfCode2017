use std::fs;

fn main()
{
	let input = fs::read_to_string("input_day1.txt").unwrap();

	let mut nums: Vec<u32> = Vec::new();

    for r in 0..input.len() - 1
    {
        nums.push(input.chars().nth(r).unwrap().to_digit(10).unwrap());
    }

    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<u32>)
{
    let mut sum: u32 = 0;
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

fn part2(nums: &Vec<u32>)
{
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
