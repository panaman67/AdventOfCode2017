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
    // open the text file and store
    let input_file = File::open("input_day12.txt").expect("No such file");
    // create buffer to read the contents from file
    let mut buf = BufReader::new(input_file);
    // output vector
    let mut output: Vec<usize> = vec![0];

    // dictionary to store all the inputs  
    let mut dict: HashMap<usize, Vec<usize>> = HashMap::new();

    // create String variable to store each line of file in
    let mut line: String;

    for _ in 0..2000 {
        // read line from file and store into line variable (String)
        // no assignment as no return value is needed
        line = "".to_owned();
        buf.read_line(&mut line).expect("Failed to read line");
        let line = line.trim();
        
        // replace unwanted tokens in string, store in String temp
        let mut temp = line.replace(" <-> ", " ");
        temp = temp.replace(",", "");
        
        // create vector of usize of each number in the line
        // includes left AND right side
        let array : Vec<usize> = temp.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        
        // create vector of "pipes" aka values the port maps to
        let mut pipes : Vec<usize> = Vec::new();
        // iterate throuh the array of values starting at 
        // first pipe (index 1) and add to pipes vector
        for spot in 1..array.len() {
            pipes.push(array[spot]);
        }
        // insert key-value pair (port, pipes) to dictionary
        dict.insert(array[0], pipes);
    }

    for x in 0.. {
        if x == output.len() {
            break;
        }

        let lol : Vec<usize> = dict[&output[x]].clone();
        for num in lol.iter() {
            if !output.contains(num) {
                output.push(*num);
            }
        }
    }
    println!("Part 1: {}", output.len());
}

fn part2()
{ 
    // open the text file and store
    let input_file = File::open("input_day12.txt").expect("No such file");
    // create buffer to read the contents from file
    let mut buf = BufReader::new(input_file);
    // output vector
    let mut output : Vec<usize> = vec![0];

    // dictionary to store all the inputs  
    let mut dict : HashMap<usize, Vec<usize>> = HashMap::new();

    // create String variable to store each line of file in
    let mut line : String;

    for _ in 0..2000
    {
        // read line from file and store into line variable (String)
        // no assignment as no return value is needed
        line = "".to_owned();
        buf.read_line(&mut line).expect("Failed to read line");
        let line = line.trim();
        
        // replace unwanted tokens in string, store in String temp
        let mut temp = line.replace(" <-> ", " ");
        temp = temp.replace(",", "");
        
        // create vector of usize of each number in the line
        // includes left AND right side
        let array : Vec<usize> = temp.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        
        // create vector of "pipes" aka values the port maps to
        let mut pipes : Vec<usize> = Vec::new();
        // iterate throuh the array of values starting at 
        // first pipe (index 1) and add to pipes vector
        for spot in 1..array.len()
        {
            pipes.push(array[spot]);
        }
        // insert key-value pair (port, pipes) to dictionary
        dict.insert(array[0], pipes);
    }
    
    let mut lol : Vec<usize>;
    let mut sum = 0;
    while dict.len() > 0
    {
        for x in 0..
        {
            if x == output.len()
            {
                sum += 1;
                break;
            }

            lol = dict[&output[x]].clone();
            for num in lol.iter()
            {
                if !output.contains(num)
                {
                    output.push(*num);
                }
            }
        }
        for y in output.iter()
        {
            dict.remove(y);
        }
        output.clear();
        if dict.len() > 0
        {
            output.push(*dict.keys().next().unwrap());
        }
    }
    println!("Part 2: {}", sum);
}
