use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main()
{
    part1();
    part2();
}

fn part1()
{
    let input_file = File::open("input_day8.txt").expect("Failed to open file");

    let mut registers : HashMap<String, i32> = HashMap::new();

    let buf = BufReader::new(&input_file);
    let mut input : Vec<String>;
    let mut instr : Vec<String>;
    let mut cond  : Vec<String>;

    for line in buf.lines()
    {
        let line = line.unwrap();
        let line = line.trim();
        
        input = line.split(" if ").map(|x| x.to_owned()).collect();
        
        instr = input[0].split(" ").map(|x| x.to_owned()).collect();
        cond  = input[1].split(" ").map(|x| x.to_owned()).collect();

        if !registers.contains_key(&instr[0])
        {
           registers.insert(instr[0].clone(), 0);
        }
        
        if !registers.contains_key(&cond[0])
        {
           registers.insert(cond[0].clone(), 0);
        }
       
        let is_good = match &cond[1][..] 
                      {
                            "<=" => { registers[&cond[0]] <= cond[2].clone().parse::<i32>().unwrap()  },
                            ">=" => { registers[&cond[0]] >= cond[2].clone().parse::<i32>().unwrap()  },
                            "==" => { registers[&cond[0]] == cond[2].clone().parse::<i32>().unwrap()  },
                            "!=" => { registers[&cond[0]] != cond[2].clone().parse::<i32>().unwrap()  },
                            ">"  => { registers[&cond[0]] > cond[2].clone().parse::<i32>().unwrap()  },
                            "<"  => { registers[&cond[0]] < cond[2].clone().parse::<i32>().unwrap()  },
                            _    => false
                      };
        if is_good
        {
            match &instr[1][..]
            {
                "inc" => {*registers.get_mut(&instr[0]).unwrap() += instr[2].clone().parse::<i32>().unwrap() },
                "dec" => {*registers.get_mut(&instr[0]).unwrap() -= instr[2].clone().parse::<i32>().unwrap() },
                _ => println!("Instr no match")
            }
        }
    }

    let mut largest : String = String::new();
    let mut num : i32 = std::i32::MIN;
    for (k,v) in registers.iter()
    {
        if v > &num
        {
            largest = k.to_owned();
            num = *v;
        }
    }
    println!("Part1: {:?}", num);
}


fn part2()
{
    let input_file = File::open("input_day8.txt").expect("Failed to open file");

    let mut registers : HashMap<String, i32> = HashMap::new();

    let buf = BufReader::new(&input_file);
    let mut input : Vec<String>;
    let mut instr : Vec<String>;
    let mut cond  : Vec<String>;

    let mut largest : String = String::new();
    let mut num : i32 = std::i32::MIN;

    for line in buf.lines()
    {
        let line = line.unwrap();
        let line = line.trim();
        
        input = line.split(" if ").map(|x| x.to_owned()).collect();
        
        instr = input[0].split(" ").map(|x| x.to_owned()).collect();
        cond  = input[1].split(" ").map(|x| x.to_owned()).collect();

        if !registers.contains_key(&instr[0])
        {
           registers.insert(instr[0].clone(), 0);
        }
        
        if !registers.contains_key(&cond[0])
        {
           registers.insert(cond[0].clone(), 0);
        }
       
        let is_good = match &cond[1][..] 
                      {
                            "<=" => { registers[&cond[0]] <= cond[2].clone().parse::<i32>().unwrap()  },
                            ">=" => { registers[&cond[0]] >= cond[2].clone().parse::<i32>().unwrap()  },
                            "==" => { registers[&cond[0]] == cond[2].clone().parse::<i32>().unwrap()  },
                            "!=" => { registers[&cond[0]] != cond[2].clone().parse::<i32>().unwrap()  },
                            ">"  => { registers[&cond[0]] > cond[2].clone().parse::<i32>().unwrap()  },
                            "<"  => { registers[&cond[0]] < cond[2].clone().parse::<i32>().unwrap()  },
                            _    => false
                      };
        if is_good
        {
            match &instr[1][..]
            {
                "inc" => {*registers.get_mut(&instr[0]).unwrap() += instr[2].clone().parse::<i32>().unwrap() },
                "dec" => {*registers.get_mut(&instr[0]).unwrap() -= instr[2].clone().parse::<i32>().unwrap() },
                _ => println!("Instr no match")
            }
        }

        for (k,v) in registers.iter()
        {
            if v > &num
            {
                largest = k.to_owned();
                num = *v;
            }
        }
    }
    println!("Part2: {:?}", num);
}
