use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main()
{
    let input_file = File::open("input_day8.txt").expect("Failed to open file");

    let mut registers : HashMap<String, i32> = HashMap::new();

    let buf = BufReader::new(&input_file);
    let input : Vec<String>;
    let instr : Vec<String>;
    let cond  : Vec<String>;

    for line in buf.lines()
    {
        let line = line.unwrap();
        let line = line.trim();
        
        input = line.split(" if ").map(|x| x.to_owned()).collect();
        
        instr = input[0].split(" ").map(|x| x.to_owned()).collect();
        cond  = input[1].split(" ").map(|x| x.to_owned()).collect();

        //let regL = registers.entry(instr[0]).or_insert(0);
        //let regR = registers.entry(cond[0]).or_insert(0);

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
        match &instr[1][..]
        {
            //"inc" => {registers.get_mut(&instr[0]).unwrap() += instr[2].clone().parse::<i32>().unwrap() },
            //"dec" => {*registers.entry(instr[0].clone()) -= instr[2].clone().parse::<i32>().unwrap() }
            _ => println!("Instr no match")
        }
        println!("{:?}, {}", registers, is_good);
        break;
    }
}
