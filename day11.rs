use std::fs::File;
use std::io::prelude::*;

fn main()
{
    // get file
    let mut input_file = File::open("input_day11.txt").expect("No file exists");
    // read to string
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Read to string failed");
    input = input.trim().to_owned();

    // parse to src vector of strings, delimit by ','
    let mut src : Vec<&str> = input.split(",").collect();
    // create dest vector of strings
    let mut dest : Vec<&str> = Vec::new();

    // while there is still substitutions to be made
    for _ in 0..50 //while src.len() != dest.len()
    {
        println!("{}", src.len());
        // get chunk iterator over src vector
        for slice in src.chunks(2)
        {
        // match with collapsable patterns eg. (ne,nw) -> n, or (ne,sw) -> NULL
            match slice
            {
                &["ne", "nw"] => dest.push("n"),  //println!("n"),
                &["nw", "ne"] => dest.push("n"),  //println!("n"),
                &["se", "sw"] => dest.push("s"),  //println!("s"),
                &["sw", "se"] => dest.push("s"),  //println!("s"),
                &["nw", "se"] => continue,        //println!("stay"),
                &["se", "nw"] => continue,        //println!("stay"),
                &["ne", "sw"] => continue,        //println!("stay"),
                &["sw", "ne"] => continue,        //println!("stay"),
                &["n", "s"] => continue,          //println!("stay"),
                &["s", "n"] => continue,          //println!("stay"),
                _ => {
                    if slice.len() == 2
                    {
                        dest.push(slice[0]);
                        dest.push(slice[1]);
                    }
                    else 
                    {
                        let temp = dest.pop().unwrap();
                        match [temp, slice[0]]
                        {
                            ["ne", "nw"] => dest.push("n"),  //println!("n"),
                            ["nw", "ne"] => dest.push("n"),  //println!("n"),
                            ["se", "sw"] => dest.push("s"),  //println!("s"),
                            ["sw", "se"] => dest.push("s"),  //println!("s"),
                            ["nw", "se"] => continue,        //println!("stay"),
                            ["se", "nw"] => continue,        //println!("stay"),
                            ["ne", "sw"] => continue,        //println!("stay"),
                            ["sw", "ne"] => continue,        //println!("stay"),
                            ["n", "s"] => continue,          //println!("stay"),
                            ["s", "n"] => continue,          //println!("stay"),
                            _ => {
                                dest.push(temp);
                                dest.push(slice[0]);
                            }
                        }
                    }
                }
            }
        }

        // clear src
        src = dest.clone();
        // move elements from dest to src
        dest.clear();
        println!("{}\n", src.len());
    }

    //println!("{:?}", src);

    // push each in pattern if no match, else push simplified direction to dest

    // swap contents of vectors

    // clear dest vector

    // repeat until no longer can simplify

    // print length of vector

}
